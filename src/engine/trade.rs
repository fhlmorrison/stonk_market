fn execute_trade(buy: &Order, sell: &Order, quantity: u64) {
    println!(
        "Trade executed: {} => {} for {} @ {}",
        buy.id, sell.id, quantity, sell.price
    );
    // todo!();
}
