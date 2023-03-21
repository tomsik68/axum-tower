use axum::{routing::get_service, Router};
use std::sync::Arc;

mod configuration;
mod endpoints;
mod http_route_populate;
mod observability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load configuration first
    let config = Arc::new(configuration::Configuration::load()?);

    observability::init_tracing(config.as_ref());

    // build the tower service serving our endpoint
    let get_hello = self::endpoints::get_hello::build(config);

    // we're building an axum app as usual, except we use get_service instead of just `get`
    let app = Router::new().route("/", get_service(get_hello));

    // make our app observable
    let app = observability::make_observable(app);

    // run it with hyper on localhost:8080
    Ok(axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?)
}
