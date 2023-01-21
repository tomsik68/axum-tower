use axum::Router;
use http::{
    header::{AsHeaderName, HeaderMap, USER_AGENT},
    version::Version,
    Request, Response,
};
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use std::any::Any;
use std::time::Duration;
use tower_http::request_id::{MakeRequestId, MakeRequestUuid, RequestId, SetRequestIdLayer};
use tower_http::trace::{MakeSpan, OnResponse, TraceLayer};
use tracing::Span;
use uuid::Uuid;

pub(crate) fn init_tracing() -> impl Any {
    use opentelemetry_otlp::WithExportConfig;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::Registry;

    // Create a new OpenTelemetry pipeline
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://otlp-opentelemetry-collector:4317");
    // Then pass it into pipeline builder
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            trace::config()
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    env!("CARGO_PKG_NAME"),
                )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default()
        .with(telemetry)
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub(crate) fn make_observable<B>(app: Router<B>) -> Router<B>
where
    B: Clone + Send + Sync + 'static,
{
    app.layer(
        TraceLayer::new_for_http()
            .make_span_with(CustomMakeSpan)
            .on_response(RecordResponseData),
    )
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid {}))
}

#[derive(Default, Clone)]
struct RecordResponseData;

#[derive(Clone)]
struct CustomMakeSpan;

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let user_agent = request.headers().header_value_for_tracing(USER_AGENT);
        let correlation_id = request
            .headers()
            .header_value_for_tracing("x-correlation-id");

        let body = request.body();

        tracing::info_span!(
            "http.server",
            otel.kind = "server",

            // TODO should be set during request parsing
            http.request_content_length = tracing::field::Empty,
            http.method = %request.method(),
            http.user_agent = user_agent,
            http.target = %request.uri(),
            http.request.header.x_correlation_id = correlation_id,
            http.response.header.x_correlation_id = correlation_id,
            http.flavor = request.version().to_flavor(),

            // optionally set by route
            http.route = tracing::field::Empty,

            // set at response time
            http.response_content_length = tracing::field::Empty,
            http.status_code = tracing::field::Empty,

            // hostname and port are hard-coded
            net.host.name = "localhost",
            net.host.port = 8080,

            // HTTPS is unsupported by this service
            http.scheme = "http",
        )
    }
}

impl<B> OnResponse<B> for RecordResponseData {
    fn on_response(self, response: &Response<B>, latency: Duration, span: &Span) {
        span.record("http.status_code", response.status().as_u16());
    }
}

trait HeaderValueForTracing {
    fn header_value_for_tracing<K: AsHeaderName>(&self, key: K) -> Option<String>;
}

impl HeaderValueForTracing for HeaderMap {
    fn header_value_for_tracing<K: AsHeaderName>(&self, key: K) -> Option<String> {
        self.get(key)
            .map(|v| v.to_str().map(ToString::to_string).ok())
            .flatten()
    }
}

trait ToFlavor {
    fn to_flavor(&self) -> Option<&'static str>;
}

impl ToFlavor for Version {
    fn to_flavor(&self) -> Option<&'static str> {
        match *self {
            Version::HTTP_09 => Some("0.9"),
            Version::HTTP_10 => Some("1.0"),
            Version::HTTP_11 => Some("1.1"),
            Version::HTTP_2 => Some("2.0"),
            Version::HTTP_3 => Some("3.0"),
            _ => None,
        }
    }
}
