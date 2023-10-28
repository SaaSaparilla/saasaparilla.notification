#![warn(clippy::all)]


use self::paths::notification::app;

mod paths;
mod routes;
mod types;

#[tokio::main]
pub(crate) async fn main() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}
