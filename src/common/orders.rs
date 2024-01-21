use super::Price;

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub user_id: u64,
    pub price: Price,
    pub quantity: u64,
    pub side: Side,
}
