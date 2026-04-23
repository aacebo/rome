use std::sync::{Arc, nonpoison::*};

use crate::state::Action;

pub struct Dispatcher<'a, TState>(RwLockWriteGuard<'a, Arc<TState>>);

impl<'a, TState> Dispatcher<'a, TState> {
    pub fn dispatch<TAction>(&mut self, action: TAction)
    where
        TAction: Action<State = TState>,
    {
        if let Some(state) = Arc::get_mut(&mut self.0) {
            action.reduce(state);
        }
    }
}

impl<'a, TState> Dispatcher<'a, TState> {
    pub(super) fn new(inner: RwLockWriteGuard<'a, Arc<TState>>) -> Self {
        Self(inner)
    }
}
