#![warn(clippy::all)]


use self::paths::notification::app;

mod paths;
mod routes;

#[tokio::main]
pub(crate) async fn main() {
    println!("Booting Server on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}
