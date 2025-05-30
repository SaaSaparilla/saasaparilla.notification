#![warn(clippy::all)]

use crate::daos::kafka::PRODUCER;
use crate::paths::notification::app;
use fastrace::collector::{Config, ConsoleReporter};
use poem::listener::TcpListener;
use settings::SETTINGS;
use std::error::Error;
use std::{io, panic};
use std::time::Duration;

mod daos;
mod paths;
mod routes;
mod settings;

#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn Error>> {
    fastrace::set_reporter(ConsoleReporter, Config::default());

    init_lazy_statics().await?;
    let settings = &SETTINGS;
    println!(
        "Booting Server on {}:{}",
        settings.server.network_interface, settings.server.port
    );
    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.server.network_interface, settings.server.port
    ));

    poem::Server::new(listener)
        .run_with_graceful_shutdown(
            app(),
            async move {
                let _ = sigterm().await;
                println!("Terminating");
            },
            Some(Duration::from_secs(1)),
        )
        .await
        .unwrap();

    // write any pending logs before exiting
    fastrace::flush();
    Ok(())
}

pub async fn sigterm() -> io::Result<()> {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?.recv().await;
    Ok(())
}

#[fastrace::trace]
async fn init_lazy_statics() -> Result<(), Box<dyn Error>> {
    assert!(panic::catch_unwind(|| &*SETTINGS).is_ok());
    assert!(panic::catch_unwind(|| &*PRODUCER).is_ok());
    Ok(())
}
