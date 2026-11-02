use std::collections::HashMap;

use super::limit::{Limit, Price};
use super::order::{Order, Side};

#[derive(Debug)]
pub struct OrderBook {
    bids: HashMap<Price, Limit>,
    asks: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn add_order(&mut self,price: f64,order: Order) {
        match order.side {
            Side::Ask => {
                let limit = self.asks.entry(Price::new(price)).or_insert(Limit::new(price));
                limit.add_order(order);
            }
            Side::Bid => {
                let limit = self.bids.entry(Price::new(price)).or_insert(Limit::new(price));
                limit.add_order(order);
            }
        }
    }
}

