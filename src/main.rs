use exchange_rs::matchengine::{Order, Side,OrderBook};

fn main() {
    let bid = Order::new(Side::Bid, 5);
    let ask = Order::new(Side::Ask, 3);
    let ask1 = Order::new(Side::Ask, 3);
    println!("{:?}", bid);
    println!("{:?}", ask);
    
    let mut orderbook = OrderBook::new();

    orderbook.add_order(10_000.25, bid);
    orderbook.add_order(10_000.25, ask);
    orderbook.add_order(10_000.25, ask1);
    println!("{:?}", orderbook);
}
