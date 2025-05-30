use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Starting");
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        sleep(std::time::Duration::from_secs(1))
    }
    println!("Terminating");
    Ok(())
}

