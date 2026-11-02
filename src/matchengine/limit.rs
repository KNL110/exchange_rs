use super::order::Order;


#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Price {
    integral: u64,
    fraction: u64,
    scaler: u64
}

impl Price {
    pub fn new(price: f64) -> Self {
        let integral: u64 = price as u64;
        let scaler: u64 = 1_00_000;
        let fraction: u64 = ((price - integral as f64) * scaler as f64) as u64;
        Price { integral, fraction, scaler }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    totalvolume: u64,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price:f64) -> Self {
        let price: Price = Price::new(price);
        Limit { price, totalvolume:0, orders: Vec::new() }
    }

    pub fn add_order(&mut self, order: Order) {
        self.totalvolume += order.size;
        self.orders.push(order);
    }
}