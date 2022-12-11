use axum::{routing::get_service, Router};


mod endpoints;
mod route_service_wrap;

#[tokio::main]
async fn main() {
    use self::route_service_wrap::RouteServiceTrait;

    // we need to build the service serving our endpoint
    let get_hello = self::endpoints::get_hello::build().inner().clone();

    // we're building an axum app as usual, except we use get_service instead of just `get`
    let app = Router::new().route("/", get_service(get_hello));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
