/// This module contains all parts of the "get_hello" endpoint.
mod endpoint_builder;
mod request;
mod service;

pub use self::endpoint_builder::build;
