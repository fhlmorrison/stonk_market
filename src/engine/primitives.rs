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
