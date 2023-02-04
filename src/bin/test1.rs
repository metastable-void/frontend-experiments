
use std::future::Future;
use frontend_experiments::{Store};

#[derive(Clone, Debug)]
struct MyState {
    count: i32,
}

#[derive(Clone, Debug)]
enum MyAction {
    Increment,
    Decrement,
}

async fn subscriber(state: &MyState) {
    println!("State changed: {:?}", state);
}

async fn another_subscriber(state: &MyState) {
    println!("State changed: {:?}", state);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::new(
        MyState { count: 0 }, 
        |state: &MyState, action: MyAction| {
            match action {
                MyAction::Increment => MyState { count: state.count + 1 },
                MyAction::Decrement => MyState { count: state.count - 1 },
            }
        }
    );
    store.subscribe(Box::new(|state| -> Box<dyn Future<Output = ()>> { Box::new(subscriber(state)) })).await;
    // store.subscribe(Box::new(|state| -> Box<dyn Future<Output = ()>> { Box::new(another_subscriber(state)) })).await;
    store.dispatch(MyAction::Increment).await;
    store.dispatch(MyAction::Increment).await;
    store.dispatch(MyAction::Decrement).await;
    println!("State: {:?}", store.get_state());
    Ok(())
}
