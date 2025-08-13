// Created by Gemini - CORRECTED VERSION
use crate::frb_generated::StreamSink; // THE FIX IS HERE: Corrected the import path.
use lazy_static::lazy_static;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

// A simple function to test the FFI bridge.
pub fn greet(name: String) -> String {
    format!("Hello from Rust, {}!", name)
}

// This function will be callable from Dart.
// It will spawn the Rust logic and stream results back.
pub fn subscribe_to_price_updates(_symbol: String, sink: StreamSink<String>) -> Result<(), anyhow::Error> {
    let (tx, mut rx) = mpsc::channel(100);

    // Spawn a Tokio task to simulate receiving data
    RUNTIME.spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        let mut price = 70000.0;
        loop {
            interval.tick().await;
            price += 10.5;
            if tx.send(price.to_string()).await.is_err() {
                break;
            }
        }
    });

    // Spawn another task to bridge the MPSC channel to the FFI sink
    RUNTIME.spawn(async move {
        while let Some(data) = rx.recv().await {
            if !sink.add(data) {
                break;
            }
        }
    });

    Ok(())
}

pub fn toggle_csv_persistence(enabled: bool) -> Result<(), anyhow::Error> {
    println!("[RUST] CSV persistence toggled to: {}", enabled);
    Ok(())
}