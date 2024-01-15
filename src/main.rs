use std::{
    collections::{binary_heap::PeekMut, BinaryHeap, HashMap, VecDeque},
    ops::Deref,
};

const FRACTIONAL_SCALAR: u64 = 100000;

#[derive(Debug, Clone, Copy)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
struct Order {
    id: u64,
    price: Price,
    quantity: u64,
    side: Side,
}

struct Limit {
    price: Price,
    orders: VecDeque<Order>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Price {
    integral: u64,
    fractional: u64,
}

impl Price {
    fn new(input: f64) -> Self {
        let integral = input as u64;
        let fractional = ((input % 1.0) * FRACTIONAL_SCALAR as f64) as u64;
        Price {
            integral,
            fractional,
        }
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{:0width$}",
            self.integral,
            self.fractional,
            width = FRACTIONAL_SCALAR.to_string().len() - 1
        )
    }
}

impl Limit {
    fn add_order(&mut self, order: Order) {
        self.orders.push_back(order);
    }
}

struct OrderBook {
    id: u64,
    buy_prices: BinaryHeap<Price>,
    buy_limits: HashMap<Price, Limit>,
    sell_prices: BinaryHeap<Price>,
    sell_limits: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new(id: u64) -> Self {
        OrderBook {
            id,
            buy_prices: BinaryHeap::new(),
            buy_limits: HashMap::new(),
            sell_prices: BinaryHeap::new(),
            sell_limits: HashMap::new(),
        }
    }

    // FIFO (Price/time priority)
    fn add_order(&mut self, mut order: Order) {
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

                                    // Execute trade at order quantity
                                    execute_trade(&order, &sell_order, order.quantity.clone());

                                    order.quantity = 0;

                                    sell_limit.orders.push_front(sell_order);
                                } else {
                                    // Lower order quantity
                                    order.quantity -= sell_order.quantity;

                                    // Execute trade at sell quantity
                                    execute_trade(&order, &sell_order, sell_order.quantity);

                                    // Remove empty limit
                                    if sell_limit.orders.is_empty() {
                                        self.sell_limits.remove(&sell_price);
                                        self.sell_prices.pop();
                                    }
                                }
                            } else {
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
                                break;
                            }
                        }
                        None => {
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

                                    // Execute trade at order quantity
                                    execute_trade(&order, &buy_order, order.quantity.clone());

                                    order.quantity = 0;

                                    buy_limit.orders.push_front(buy_order);
                                } else {
                                    // Lower order quantity
                                    order.quantity -= buy_order.quantity;

                                    // Execute trade at sell quantity
                                    execute_trade(&order, &buy_order, buy_order.quantity);

                                    // Remove empty limit
                                    if buy_limit.orders.is_empty() {
                                        self.buy_limits.remove(&buy_price);
                                        self.buy_prices.pop();
                                    }
                                }
                            } else {
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
                                break;
                            }
                        }
                        None => {
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
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn execute_trade(buy: &Order, sell: &Order, quantity: u64) {
    println!(
        "Trade executed: {} => {} for {} @ {}",
        buy.id, sell.id, quantity, sell.price
    );
    // todo!();
}

fn main() {
    println!("STONKS!");
    // create heap

    let buy = Order {
        id: 1,
        price: Price::new(100.05),
        quantity: 100,
        side: Side::Buy,
    };

    let sell = Order {
        id: 2,
        price: Price::new(99.95),
        quantity: 100,
        side: Side::Sell,
    };

    let mut orderbook = OrderBook::new(1);

    println!("sell then buy");
    orderbook.add_order(sell.clone());
    orderbook.add_order(buy.clone());

    println!("buy then sell");
    orderbook.add_order(buy.clone());
    orderbook.add_order(sell.clone());
}
