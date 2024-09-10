# First of all

### i'm rather use `impl fmt::Debug` for ignoring `action` in struct while print `Order` struct instead of `#[derive(Debug)]`
```rust
struct Order {
    id: u64,
    price: f64,
    action: bool, //action buy=true || action sell=false, i'd like to put "action" word for buy or sell
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
```

# Second Things

### i'd like to use different function for serve the order and make it returning as a stream like:
```rust
async fn serve_orders(order_list: Vec<Order>) -> impl Stream<Item = Order> {
    stream! {
        for order in order_list {
            let delay_duration = rand::thread_rng().gen_range(500..900);
            sleep(Duration::from_millis(delay_duration)).await;
            yield order;
        }
    }
}
```
### instead of merge `stream!` macros in `execute_orders` function, but in other words i can do it like:
```rust
async fn execute_orders(order_list: Vec<Order>) {
    //accept vector directly
    let order_stream = stream! {
        for order in order_list {
            let delay_duration = rand::thread_rng().gen_range(500..900);
            sleep(Duration::from_millis(delay_duration)).await;
            yield order;
        }
    }
    futures::pin_mut!(order_stream); //pin the macros variable
    //continue code
    }
```