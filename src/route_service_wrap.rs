/// This module contains some glue code that allows returning an `impl` type from the
/// [`crate::endpoints::get_hello::endpoint_builder::build`] function.
///
/// That is done by combining the following things:
///
/// 1. We create a new type RouteServiceWrap with some generics and constraints. The constraints
///    are heavily influenced by [axum::routing::method_routing::any_service](https://docs.rs/axum/latest/axum/routing/method_routing/fn.any_service.html).
/// 2. We extract some constraints into the RouteService type to make the struct declaration
///    easier to read and also possible to parse.
/// 3. We use [degeneric-macros](https://docs.rs/degeneric-macros) to generate a trait and hide
///    all those generics into associated types.
///
/// Our endpoint builder may now return `impl RouteServiceTrait`. That's a type that contains a
/// Service satisfying all the constraints given by axum to a tower::Service used for routing
/// requests.

#[derive(degeneric_macros::Degeneric, typed_builder::TypedBuilder)]
#[degeneric(trait = "pub trait RouteServiceTrait")]
pub struct RouteServiceWrap<
    Fut: Send,
    Resp: axum::response::IntoResponse,
    Inner: RouteService<Fut, Resp>,
> {
    #[builder(default)]
    _pd: std::marker::PhantomData<(Fut, Resp)>,
    inner: Inner,
}

trait_set::trait_set! {
    pub trait RouteService<Fut: Send, Resp: axum::response::IntoResponse> =
        tower::Service<axum::http::Request<hyper::Body>, Response = Resp, Future = Fut, Error = std::convert::Infallible> + Send + Clone;
}
