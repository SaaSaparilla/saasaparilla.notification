use crate::routes;
use fastrace_poem;
use poem::{Endpoint, EndpointExt, Route};
use poem::{get, post};

pub fn app() -> impl Endpoint {
    Route::new()
        .at("/healthz", get(routes::health))
        // TODO: add appropriate request validation to k8s Ingress and CiliumNetworkPolicy objects
        .at(
            "/api/v1/notification",
            post(routes::notification::create_v1),
        )
        .with(fastrace_poem::FastraceMiddleware)
}
