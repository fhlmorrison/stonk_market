use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::common::Order;
use crate::common::Price;
use crate::common::Side;
use crate::common::TickerSymbol;
use crate::common::Trade;

struct Limit {
    price: Price,
    orders: VecDeque<Order>,
}

impl Limit {
    fn add_order(&mut self, order: Order) {
        self.orders.push_back(order);
    }
    fn remove_order(&mut self, order: &Order) -> Option<Order> {
        if let Some((index, removed_order)) = self
            .orders
            .iter()
            .enumerate()
            .find(|(i, o)| o.user_id == order.user_id && o.timestamp == order.timestamp)
            .map(|(i, o)| (i, o.clone()))
        {
            self.orders.remove(index);
            Some(removed_order)
        } else {
            None
        }
    }
}

pub struct OrderBook {
    pub symbol: TickerSymbol,
    buy_prices: BinaryHeap<Price>,
    buy_limits: HashMap<Price, Limit>,
    sell_prices: BinaryHeap<Price>,
    sell_limits: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new(symbol: &str) -> Self {
        OrderBook {
            symbol: TickerSymbol::new(symbol),
            buy_prices: BinaryHeap::new(),
            buy_limits: HashMap::new(),
            sell_prices: BinaryHeap::new(),
            sell_limits: HashMap::new(),
        }
    }

    fn insert_buy_order(&mut self, order: Order) {
        if !self.buy_limits.contains_key(&order.price) {
            self.buy_prices.push(order.price.clone());
        }
        self.buy_limits
            .entry(order.price)
            .or_insert(Limit {
                price: order.price.clone(),
                orders: VecDeque::new(),
            })
            .add_order(order);
    }

    fn insert_sell_order(&mut self, order: Order) {
        if !self.sell_limits.contains_key(&order.price) {
            self.sell_prices.push(order.price.clone());
        }
        self.sell_limits
            .entry(order.price)
            .or_insert(Limit {
                price: order.price.clone(),
                orders: VecDeque::new(),
            })
            .add_order(order);
    }

    // FIFO (Price/time priority)
    pub fn add_order(&mut self, mut order: Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        match order.side {
            Side::Buy => {
                while order.quantity > 0 {
                    let lowest_sell_price = self.sell_prices.peek().map(|peek| peek.clone());
                    match lowest_sell_price {
                        Some(sell_price) => {
                            if order.price >= sell_price {
                                let sell_limit = self.sell_limits.get_mut(&sell_price).unwrap();
                                let mut sell_order = sell_limit.orders.pop_front().unwrap();
                                // Buy max quantity available
                                if sell_order.quantity > order.quantity {
                                    // Lower sell quantity
                                    sell_order.quantity -= order.quantity;

                                    let trade_quantity = order.quantity.clone();

                                    // Execute trade at order quantity
                                    execute_trade(self.symbol, &order, &sell_order, trade_quantity);
                                    trades.push(Trade::from_orders(
                                        &order,
                                        &sell_order,
                                        trade_quantity,
                                    ));

                                    order.quantity = 0;

                                    sell_limit.orders.push_front(sell_order);
                                } else {
                                    // Lower order quantity
                                    order.quantity -= sell_order.quantity;

                                    let trade_quantity = sell_order.quantity.clone();

                                    // Execute trade at sell quantity
                                    execute_trade(self.symbol, &order, &sell_order, trade_quantity);
                                    trades.push(Trade::from_orders(
                                        &order,
                                        &sell_order,
                                        trade_quantity,
                                    ));

                                    // Remove empty limit
                                    if sell_limit.orders.is_empty() {
                                        self.sell_limits.remove(&sell_price);
                                        self.sell_prices.pop();
                                    }
                                }
                            } else {
                                self.insert_buy_order(order);
                                break;
                            }
                        }
                        None => {
                            self.insert_buy_order(order);
                            break;
                        }
                    }
                }
            }
            Side::Sell => {
                while order.quantity > 0 {
                    let highest_buy_price = self.buy_prices.peek().map(|peek| peek.clone());
                    match highest_buy_price {
                        Some(buy_price) => {
                            if order.price <= buy_price {
                                let buy_limit = self.buy_limits.get_mut(&buy_price).unwrap();
                                let mut buy_order = buy_limit.orders.pop_front().unwrap();
                                // Buy max quantity available
                                if buy_order.quantity > order.quantity {
                                    // Lower sell quantity
                                    buy_order.quantity -= order.quantity;

                                    let trade_quantity = order.quantity.clone();

                                    // Execute trade at order quantity
                                    execute_trade(self.symbol, &order, &buy_order, trade_quantity);
                                    trades.push(Trade::from_orders(
                                        &order,
                                        &buy_order,
                                        trade_quantity,
                                    ));

                                    order.quantity = 0;

                                    buy_limit.orders.push_front(buy_order);
                                } else {
                                    // Lower order quantity
                                    order.quantity -= buy_order.quantity;

                                    let trade_quantity = order.quantity.clone();

                                    // Execute trade at sell quantity
                                    execute_trade(
                                        self.symbol,
                                        &order,
                                        &buy_order,
                                        buy_order.quantity,
                                    );
                                    trades.push(Trade::from_orders(
                                        &order,
                                        &buy_order,
                                        trade_quantity,
                                    ));

                                    // Remove empty limit
                                    if buy_limit.orders.is_empty() {
                                        self.buy_limits.remove(&buy_price);
                                        self.buy_prices.pop();
                                    }
                                }
                            } else {
                                self.insert_sell_order(order);
                                break;
                            }
                        }
                        None => {
                            self.insert_sell_order(order);
                            break;
                        }
                    }
                }
            }
        }
        return trades;
    }

    pub fn clear(&mut self) {
        self.buy_limits.drain().for_each(|(price, limit)| {
            limit
                .orders
                .into_iter()
                .for_each(|order| refund_order(&order));
        })
    }

    fn cancel_order(&mut self, order: &Order) -> Result<Order, ()> {
        match order.side {
            Side::Buy => {
                if let Some(limit) = self.buy_limits.get_mut(&order.price) {
                    if let Some(removed_order) = limit.remove_order(order) {
                        refund_order(&removed_order);
                        return Ok(removed_order);
                    }
                }
            }
            Side::Sell => {
                if let Some(limit) = self.sell_limits.get_mut(&order.price) {
                    if let Some(removed_order) = limit.remove_order(order) {
                        refund_order(&removed_order);
                        return Ok(removed_order);
                    }
                }
            }
        }
        Err(())
    }
}

fn execute_trade(symbol: TickerSymbol, buy: &Order, sell: &Order, quantity: u64) {
    println!(
        "Trade executed: {} {} @ {:.2} ({} => {})",
        symbol, quantity, sell.price, buy.user_id, sell.user_id
    );
    // todo!();
}

fn refund_order(order: &Order) {
    println!(
        "Order refunded: {} {} @ {}",
        order.user_id, order.quantity, order.price
    );
}

#[cfg(test)]
mod matching_tests {
    use crate::common::TimeStamp;

    use super::*;

    #[test]
    fn test() {
        let mut orderbook = OrderBook::new("STNK");
        let buy = Order {
            user_id: 1,
            timestamp: TimeStamp::new(),
            price: Price::try_from("100.05").unwrap(),
            quantity: 100,
            side: Side::Buy,
        };
        let sell = Order {
            user_id: 2,
            timestamp: TimeStamp::new(),
            price: Price::try_from("99.95").unwrap(),
            quantity: 100,
            side: Side::Sell,
        };

        println!("{:?}", buy.price);

        orderbook.add_order(sell.clone());
        orderbook.add_order(buy.clone());
        orderbook.add_order(buy.clone());
        orderbook.add_order(sell.clone());
        println!("test");
    }

    #[test]
    fn test_clear() {
        let mut orderbook = OrderBook::new("STNK");
        let buy = Order {
            user_id: 1,
            timestamp: TimeStamp::new(),
            price: Price::try_from("100.05").unwrap(),
            quantity: 100,
            side: Side::Buy,
        };
        let sell = Order {
            user_id: 2,
            timestamp: TimeStamp::new(),
            price: Price::try_from("99.95").unwrap(),
            quantity: 100,
            side: Side::Sell,
        };

        println!("{:?}", buy.price);

        orderbook.add_order(sell.clone());
        orderbook.add_order(buy.clone());
        println!("test");

        orderbook.clear();
    }

    #[test]
    fn test_ticker_symbol() {
        let symbol = TickerSymbol::new("STNK");
        assert_eq!(symbol.to_string(), "STNK");

        let symbol = TickerSymbol::new("STN");
        assert_eq!(symbol.to_string(), "STN");

        let symbol = TickerSymbol::new("ST");
        assert_eq!(symbol.to_string(), "ST");

        let symbol = TickerSymbol::new("STNKY");
        assert_eq!(symbol.to_string(), "STNK");
    }
}
