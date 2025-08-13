// Created by Gemini - FINAL CORRECTED VERSION
use super::{AdapterError, ExchangeAdapter};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt, SinkExt};
use serde_json::{json, Value};
use std::pin::Pin;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const COINBASE_WS_URL: &str = "wss://ws-feed.exchange.coinbase.com";

// ====================================================================
// THE FIX IS HERE: Changed from a unit struct `struct CoinbaseAdapter;`
// to a regular struct with an empty body `struct CoinbaseAdapter {}`.
// This makes it parsable by the FFI code generator.
// ====================================================================
pub struct CoinbaseAdapter {}

#[async_trait]
impl ExchangeAdapter for CoinbaseAdapter {
    fn venue_name(&self) -> &'static str { "Coinbase" }

    async fn subscribe(
        &self,
        symbols: &[String],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Value, AdapterError>> + Send>>, AdapterError> {
        let (ws_stream, _) = connect_async(COINBASE_WS_URL)
            .await
            .map_err(|e| AdapterError::NetworkError(e.to_string()))?;

        let (mut write, read) = ws_stream.split();

        let product_ids: Vec<String> = symbols.iter().map(|s| s.replace("/", "-")).collect();
        let subscribe_msg = json!({
            "type": "subscribe",
            "product_ids": product_ids,
            "channels": ["matches"]
        });

        write.send(Message::Text(subscribe_msg.to_string()))
            .await
            .map_err(|e| AdapterError::NetworkError(e.to_string()))?;

        let stream = read.filter_map(|msg| async {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<Value>(&text) {
                        Ok(data) if data["type"] == "match" => Some(Ok(data)),
                        Ok(_) => None, // Ignore other message types like heartbeats
                        Err(e) => Some(Err(AdapterError::DeserializationError(e.to_string()))),
                    }
                }
                Err(e) => Some(Err(AdapterError::NetworkError(e.to_string()))),
                _ => None,
            }
        });

        Ok(Box::pin(stream))
    }
}