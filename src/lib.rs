
use std::future::Future;
use std::pin::Pin;

pub type Reducer<State, Action> = fn(&State, Action) -> State;

pub trait Subscriber<State> {
    fn call(&self, state: &State) -> Box<dyn Future<Output = ()>>;
}

impl<State, F> Subscriber<State> for F
where
    F: Fn(&State) -> Box<dyn Future<Output = ()>>,
{
    fn call(&self, state: &State) -> Box<dyn Future<Output = ()>> {
        self(state)
    }
}

pub struct Store<State, Action>
where
    State: Clone,
{
    state: State,
    reducer: Reducer<State, Action>,
    subscribers: Vec<Box<dyn Subscriber<State>>>,
}

impl<State, Action> Store<State, Action>
where
    State: Clone,
{
    pub fn new(initial_state: State, reducer: Reducer<State, Action>) -> Self {
        Self { state: initial_state, reducer, subscribers: vec![] }
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub async fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);

        for subscriber in &self.subscribers {
            Pin::from(subscriber.call(&self.state)).await;
        }
    }

    pub async fn subscribe(&mut self, subscriber: Box<dyn Subscriber<State>>) {
        Pin::from(subscriber.call(&self.state)).await;
        self.subscribers.push(subscriber);
    }
}
