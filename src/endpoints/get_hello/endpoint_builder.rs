use crate::http_route_populate::*;
use tower::ServiceBuilder;

use super::request::HelloRequestLayer;
use super::service::HelloService;

/// This function takes all the layers that make up the endpoint and uses the ServiceBuilder to
/// assemble them into a service in the right order.
///
/// If any service-wide shared state is required, it can be passed as a parameter to this
/// function. In turn, this function shall pass the state to the right layers which pass it to
/// the right services.
///
/// Observe that we return an `impl` type to hide the module's implementation details.
pub fn build() -> impl crate::route_service_wrap::RouteServiceTrait {
    // Here, we wrap the service inside a structure called RouteServiceWrap.
    crate::route_service_wrap::RouteServiceWrap::builder()
        .inner(
            ServiceBuilder::new()
                .layer(http_route_populate("/"))
                .layer(HelloRequestLayer::default())
                .service(HelloService::default()),
        )
        .build()
}
