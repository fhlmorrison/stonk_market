use std::{sync::mpsc::{self, Receiver, Sender}, thread};

use crate::common::Order;

use super::matching::OrderBook;



pub struct OrderBookHandle {
    input: Sender<Order>,
    output: Receiver<i32>,
}

impl OrderBookHandle {
    pub fn spawn(ticker: &str)-> OrderBookHandle {
        let (orderIn, orderOut) = mpsc::channel();
        let (tradeIn, tradeOut) = mpsc::channel();

        let mut thread = OrderBookThread {
            input: orderOut,
            output: tradeIn,
            orderBook: OrderBook::new(ticker)
        };

        thread::spawn(move || thread.listen());


        OrderBookHandle{
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
    output: Sender<i32>,
    orderBook: OrderBook,
}

impl OrderBookThread {

    fn listen(&mut self) {
        loop {
            let res = self.input.recv();
           match res {
            Ok(order) => self.orderBook.add_order(order),
            Err(_) => break
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
}