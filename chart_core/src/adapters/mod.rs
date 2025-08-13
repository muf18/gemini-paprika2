// Created by Gemini
use async_trait::async_trait;
use futures_util::stream::Stream;
use serde_json::Value;
use std::pin::Pin;

pub mod coinbase;

#[derive(Debug)]
pub enum AdapterError {
    NetworkError(String),
    DeserializationError(String),
}

#[async_trait]
pub trait ExchangeAdapter: Send + Sync {
    fn venue_name(&self) -> &'static str;
    async fn subscribe(
        &self,
        symbols: &[String],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Value, AdapterError>> + Send>>, AdapterError>;
}