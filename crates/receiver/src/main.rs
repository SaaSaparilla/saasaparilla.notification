#![warn(clippy::all)]

use crate::daos::kafka::PRODUCER;
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
    init_lazy_statics().await?;
    let settings = SETTINGS.get().ok_or("")?;
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

async fn init_lazy_statics() -> Result<(), Box<dyn Error>> {
    SETTINGS.get().ok_or("")?;
    PRODUCER.get().ok_or("")?;
    Ok(())
}
