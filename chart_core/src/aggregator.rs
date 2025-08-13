// Created by Gemini
// NOTE: This is a simplified placeholder. A full implementation would handle
// multiple timeframes and use the Protobuf models.

use rust_decimal::Decimal;
use std::collections::VecDeque;
use std::str::FromStr;
use std::time::Duration;

// A single data point for VWAP calculation
struct VwapPoint {
    price: Decimal,
    volume: Decimal,
    timestamp: chrono::DateTime<chrono::Utc>,
}

// Manages aggregation for a single symbol and timeframe
pub struct TimeframeAggregator {
    timeframe: Duration,
    window: VecDeque<VwapPoint>,
    sum_price_volume: Decimal,
    sum_volume: Decimal,
    current_vwap: Decimal,
}

impl TimeframeAggregator {
    pub fn new(timeframe_str: &str) -> Self {
        let timeframe = match timeframe_str {
            "1m" => Duration::from_secs(60),
            "5m" => Duration::from_secs(300),
            _ => Duration::from_secs(60),
        };
        Self {
            timeframe,
            window: VecDeque::new(),
            sum_price_volume: Decimal::ZERO,
            sum_volume: Decimal::ZERO,
            current_vwap: Decimal::ZERO,
        }
    }

    pub fn add_trade(&mut self, price_str: &str, size_str: &str, timestamp_rfc3339: &str) {
        let price = Decimal::from_str(price_str).unwrap_or_default();
        let volume = Decimal::from_str(size_str).unwrap_or_default();
        let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_rfc3339)
            .unwrap()
            .with_timezone(&chrono::Utc);

        let new_point = VwapPoint { price, volume, timestamp };
        self.window.push_back(new_point);

        self.sum_price_volume += price * volume;
        self.sum_volume += volume;

        let cutoff = timestamp - self.timeframe;
        while let Some(front) = self.window.front() {
            if front.timestamp < cutoff {
                if let Some(old_point) = self.window.pop_front() {
                    self.sum_price_volume -= old_point.price * old_point.volume;
                    self.sum_volume -= old_point.volume;
                }
            } else {
                break;
            }
        }

        if !self.sum_volume.is_zero() {
            self.current_vwap = self.sum_price_volume / self.sum_volume;
        }
    }

    pub fn get_vwap(&self) -> Decimal {
        self.current_vwap
    }

    pub fn get_cumulative_volume(&self) -> Decimal {
        self.sum_volume
    }
}