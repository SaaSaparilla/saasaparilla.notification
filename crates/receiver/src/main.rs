#![warn(clippy::all)]

use self::paths::notification::app;
use crate::settings::SETTINGS;
use std::error::Error;

mod paths;
mod routes;
mod settings;

#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn Error>> {
    let settings = SETTINGS.get().unwrap();
    println!(
        "Booting Server on {}:{}",
        settings.server.network_interface, settings.server.port
    );
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app().into_make_service())
        .await
        .unwrap();
    Ok(())
}
