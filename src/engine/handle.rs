use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::common::{Order, TickerSymbol, Trade};

use super::matching::OrderBook;

pub struct OrderBookHandle {
    symbol: TickerSymbol,
    input: Sender<Order>,
    output: Receiver<Trade>,
}

impl OrderBookHandle {
    pub fn spawn(ticker: &str) -> OrderBookHandle {
        let (orderIn, orderOut) = mpsc::channel();
        let (tradeIn, tradeOut) = mpsc::channel();

        let mut thread = OrderBookThread {
            input: orderOut,
            output: tradeIn,
            orderBook: OrderBook::new(ticker),
        };

        let symbol = thread.orderBook.symbol.clone();

        thread::spawn(move || thread.listen());

        OrderBookHandle {
            symbol: symbol,
            input: orderIn,
            output: tradeOut,
        }
    }

    pub fn add_order(&self, order: Order) {
        self.input.send(order);
    }
}

struct OrderBookThread {
    input: Receiver<Order>,
    output: Sender<Trade>,
    orderBook: OrderBook,
}

impl OrderBookThread {
    fn listen(&mut self) {
        loop {
            let res = self.input.recv();
            match res {
                Ok(order) => {
                    let trades = self.orderBook.add_order(order);
                    for trade in trades {
                        let res2 = self.output.send(trade);
                        if res2.is_err() {
                            // TODO add better error handling here
                            eprintln!("Trade failed to send");
                        }
                    }
                }
                Err(_) => break,
            }
        }
        // Cleanup and exit
        self.orderBook.clear();
        println!("Orderbook for {} exiting", self.orderBook.symbol)
    }
}

#[cfg(test)]
mod handle_test {
    use crate::common::{Order, Price, Side, TimeStamp};

    use super::OrderBookHandle;

    #[test]
    fn test_send() {
        let handle = OrderBookHandle::spawn("AAAA".into());

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

        handle.input.send(buy);
        handle.input.send(sell);
    }

    #[test]
    fn test_multiple() {
        let handle1 = OrderBookHandle::spawn("AAAA".into());
        let handle2 = OrderBookHandle::spawn("BBBB".into());

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

        handle1.input.send(buy);
        handle1.input.send(sell);
        handle2.input.send(buy);
        handle2.input.send(sell);
    }
}
