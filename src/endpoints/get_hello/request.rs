/// This module contains a the HelloRequestLayer which performs request parsing.
///
/// This doesn't have anything interesting, but it can serve as an example of "hello world"
/// middleware for tower beginners.
/// For this middleware, we need two main components:
///
/// - a Service implementation which performs the request transformation
/// - a Layer implementation which attaches the middleware to an existing service
use axum::http::Request;

use hyper::Body;
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
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Request<Body>) -> Self::Future {
        // The request parsing itself is left as an exercise for the reader
        let hr = HelloRequest {};
        self.inner.call(hr)
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
