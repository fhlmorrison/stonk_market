use super::Price;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct TimeStamp(u128);

impl TimeStamp {
    pub fn new() -> Self {
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH);
        let nanos = unix_time.map(|e| e.as_nanos()).unwrap_or(0);
        TimeStamp(nanos)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub user_id: u64,
    pub timestamp: TimeStamp,
    pub price: Price,
    pub quantity: u64,
    pub side: Side,
}
