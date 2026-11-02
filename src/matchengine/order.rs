#[derive(Debug)]
pub enum Side {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct Order {
    pub side: Side,
    pub size: u64,
}


impl Order {
    pub fn new(side: Side, size: u64) -> Self {
        Order { side, size }
    }
}