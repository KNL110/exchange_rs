use exchange_rs::matchengine::{Limit, Order, Side};

fn main() {
    let bid = Order::new(Side::Bid, 5);
    let ask = Order::new(Side::Ask, 3);
    println!("{:?}", bid);
    println!("{:?}", ask);

    let mut limit = Limit::new(10_000.25, 0);
    println!("{:?}", limit);
    limit.add_order(bid);
    println!("{:?}", limit);
}
