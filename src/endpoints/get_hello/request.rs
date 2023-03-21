/// This module contains a the HelloRequestLayer which parses the request and returns an
/// appropriate response.
///
/// This doesn't have anything interesting, but it can serve as an example of "hello world"
/// middleware for tower beginners.
/// For this middleware, we need two main components:
///
/// - a Service implementation which performs the request transformation
/// - a Layer implementation which attaches the middleware to an existing service
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use futures::future::BoxFuture;
use hyper::Body;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::task::Context;
use std::task::Poll;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct HelloRequestService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for HelloRequestService<S>
where
    S: Service<HelloRequest>,
    S::Future: Send + 'static,
    S::Response: IntoResponse,
    S::Error: IntoResponse,
{
    type Response = Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Request<Body>) -> Self::Future {
        // The request parsing itself is left as an exercise for the reader.
        // Sidenote: `HelloRequest: Send` is required
        let hr = HelloRequest {};

        // `fut` is moved into the closure, so it needs to by Send + 'static
        let fut = self.inner.call(hr);

        Box::pin(async move {
            let resp = fut.await;
            Ok(match resp {
                Ok(x) => x.into_response(),
                Err(err) => err.into_response(),
            })
        })
    }
}

#[derive(Default)]
pub struct HelloRequestLayer<S> {
    _s: PhantomData<S>,
}

impl<S> Layer<S> for HelloRequestLayer<S> {
    type Service = HelloRequestService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HelloRequestService { inner }
    }
}

pub struct HelloRequest;
