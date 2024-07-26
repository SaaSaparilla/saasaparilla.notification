#![warn(clippy::all)]

use std::error::Error;
use std::panic;
use tokio::net::TcpListener;
use settings::SETTINGS;

use crate::daos::kafka::PRODUCER;
use crate::paths::notification::app;

mod daos;
mod paths;
mod routes;
mod settings;

#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn Error>> {
    init_lazy_statics().await?;
    let settings = &SETTINGS;
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
    assert!(panic::catch_unwind(|| &*SETTINGS).is_ok());
    assert!(panic::catch_unwind(|| &*PRODUCER).is_ok());
    Ok(())
}
