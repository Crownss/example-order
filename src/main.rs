use async_stream::stream;
use futures::{stream::Stream, StreamExt}; // Tambahkan StreamExt untuk menggunakan next()
use rand::Rng;
use std::{collections::VecDeque, fmt};
use tokio::time::{sleep, Duration};

struct Order {
    id: u64,
    price: f64,
    action: bool, //action buy=true || action sell=false same as pdf with diff name i'd like to put "action" for buy or sell
}
impl fmt::Debug for Order {
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        formater
            .debug_struct("Order")
            .field("id", &self.id)
            .field("price", &self.price)
            .finish()
    }
}

async fn generate_orders() -> Vec<Order> {
    vec![
        Order {
            id: 1,
            price: 100.0,
            action: true,
        },
        Order {
            id: 2,
            price: 105.0,
            action: true,
        },
        Order {
            id: 3,
            price: 95.0,
            action: false,
        },
        Order {
            id: 4,
            price: 90.0,
            action: false,
        },
        Order {
            id: 5,
            price: 90.0,
            action: false,
        },
        Order {
            id: 6,
            price: 100.0,
            action: false,
        },
        Order {
            id: 7,
            price: 80.0,
            action: true,
        },
        Order {
            id: 8,
            price: 112.0,
            action: true,
        },
        Order {
            id: 9,
            price: 110.0,
            action: true,
        },
        Order {
            id: 10,
            price: 85.0,
            action: true,
        },
    ]
}

fn serve_orders(order_list: Vec<Order>) -> impl Stream<Item = Order> {
    stream! {
        for order in order_list {
            let delay_duration = rand::thread_rng().gen_range(500..900);
            sleep(Duration::from_millis(delay_duration)).await;
            yield order;
        }
    }
}

async fn execute_orders(get_order_stream: impl Stream<Item = Order>) {
    futures::pin_mut!(get_order_stream);
    let mut buy_orders: Vec<Order> = Vec::new();
    let mut sell_orders: Vec<Order> = Vec::new();
    while let Some(order) = get_order_stream.next().await {
        let mut printing = String::from(format!("Received: something {:?}", order));
        let mut is_match = true;
        let mut matched = String::from(
            "Matched: Buy Order buyorderid and Sell Order sellorderid at price pricing",
        );
        if order.action {
            printing = printing.replace("something", "Buy");
            if let Some(idx) = sell_orders
                .iter()
                .position(|sell| sell.price <= order.price)
            {
                let matched_sell = sell_orders.remove(idx);
                matched = matched.replace("buyorderid", format!("{}", order.id).as_str());
                matched = matched.replace("sellorderid", format!("{}", matched_sell.id).as_str());
                matched = matched.replace("pricing", format!("{}", order.price).as_str());
            } else {
                buy_orders.push(order);
                is_match = false;
            }
        } else {
            printing = printing.replace("something", "Sell");
            if let Some(idx) = buy_orders.iter().position(|buy| buy.price >= order.price) {
                let matched_buy = buy_orders.remove(idx);
                matched = matched.replace("buyorderid", format!("{}", matched_buy.id).as_str());
                matched = matched.replace("sellorderid", format!("{}", order.id).as_str());
                matched = matched.replace("pricing", format!("{}", matched_buy.price).as_str());
            } else {
                sell_orders.push(order);
                is_match = false;
            }
        }
        println!("{}", printing);
        if is_match {
            println!("{}", matched);
        }
    }
}

#[tokio::main]
async fn main() {
    let orders = generate_orders().await;
    let serves = serve_orders(orders);
    execute_orders(serves).await;
}
