#![warn(clippy::all)]

use crate::paths::notification::app;
use crate::settings::SETTINGS;
use std::error::Error;
use tokio::net::TcpListener;

mod daos;
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
    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.server.network_interface, settings.server.port
    ))
    .await
    .unwrap();
    axum::serve(listener, app()).await.unwrap();
    Ok(())
}
