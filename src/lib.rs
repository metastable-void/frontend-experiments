
use std::future::Future;
use std::pin::Pin;

pub type Reducer<State, Action> = fn(&State, Action) -> State;

pub struct Store<State, Action, Sub>
where
    State: Clone,
    Sub: for<'a> Fn(&'a State) -> Box<dyn Future<Output = ()> + 'a>,
{
    state: State,
    reducer: Reducer<State, Action>,
    subscribers: Vec<Box<Sub>>,
}

impl<State, Action, Sub> Store<State, Action, Sub>
where
    State: Clone,
    Sub: for<'a> Fn(&'a State) -> Box<dyn Future<Output = ()> + 'a>,
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
            Pin::from(subscriber(&self.state)).await;
        }
    }

    pub async fn subscribe(&mut self, subscriber: Box<Sub>) {
        Pin::from(subscriber(&self.state)).await;
        self.subscribers.push(subscriber);
    }
}
