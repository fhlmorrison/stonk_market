use super::{Order, Price, TimeStamp};

pub struct Trade {
    timestamp: TimeStamp,
    quantity: u64,
    price: Price,
    buy_user: u64,
    sell_user: u64,
}

impl Trade {
    pub fn from_orders(buy: &Order, sell: &Order, quantity: u64) -> Trade {
        Trade {
            timestamp: TimeStamp::new(),
            quantity: quantity,
            price: buy.price,
            buy_user: buy.user_id,
            sell_user: sell.user_id
        }
    }
}