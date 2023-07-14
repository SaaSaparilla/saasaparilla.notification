#![warn(clippy::all)]

use saasparilla_notification_receiver::*;

#[tokio::main]
pub(crate) async fn main() {

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}


