
use frontend_experiments::{Store, BoxedEmptyFuture};

#[derive(Clone, Debug)]
struct MyState {
    count: i32,
}

#[derive(Clone, Debug)]
enum MyAction {
    Increment,
    Decrement,
}

async fn subscriber(state: MyState) {
    println!("State changed: {:?}", state);
}

fn yet_another_subscriber(state: MyState) -> BoxedEmptyFuture {
    Box::new(async move {
        println!("State changed, haha: {:?}", state);
    })
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
    store.subscribe(Box::new(|state: MyState| -> BoxedEmptyFuture { Box::new(subscriber(state)) })).await;
    store.subscribe(Box::new(yet_another_subscriber)).await;
    store.dispatch(MyAction::Increment).await;
    store.dispatch(MyAction::Increment).await;
    store.dispatch(MyAction::Decrement).await;
    println!("State: {:?}", store.get_state());
    Ok(())
}
