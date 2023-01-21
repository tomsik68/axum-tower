//! This module takes care of populating the http.route tracing span tag using the
//! configured value.

use axum::http::Request;
use hyper::Body;
use tower::{Layer, Service};

pub fn http_route_populate(route: &'static str) -> HttpRoutePopulateLayer {
    HttpRoutePopulateLayer { route }
}

#[derive(Clone)]
pub struct HttpRoutePopulateLayer {
    route: &'static str,
}

impl HttpRoutePopulateLayer {
    pub fn new(route: &'static str) -> Self {
        Self { route }
    }
}

impl<S> Layer<S> for HttpRoutePopulateLayer {
    type Service = HttpRoutePopulateService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HttpRoutePopulateService {
            inner,
            route: self.route.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct HttpRoutePopulateService<S> {
    inner: S,
    route: String,
}

impl<S> Service<Request<Body>> for HttpRoutePopulateService<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        tracing::span::Span::current().record("http.route", &self.route);
        self.inner.call(req)
    }
}
