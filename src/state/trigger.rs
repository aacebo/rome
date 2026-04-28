use crate::state::Action;

pub trait Trigger<TState>: Send + Sync + 'static {
    fn execute(&self, state: &TState, action: &dyn Action<State = TState>);
}

impl<T, F: Fn(&T, &dyn Action<State = T>) + Send + Sync + 'static> Trigger<T> for F {
    fn execute(&self, state: &T, action: &dyn Action<State = T>) {
        (self)(state, action);
    }
}
