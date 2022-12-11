/// This module contains a simple hello world tower service.
use futures::future::Ready;
use std::convert::Infallible;
use std::task::{Context, Poll};
use tower::Service;

use super::request::HelloRequest;

#[derive(Default, Clone)]
pub struct HelloService {}

impl Service<HelloRequest> for HelloService {
    type Response = &'static str;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: HelloRequest) -> Self::Future {
        futures::future::ready(Ok("hello"))
    }
}
