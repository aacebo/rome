use std::sync::{Arc, nonpoison::*};

use crate::state::{Accessor, Dispatcher};

/// Central coordinator that owns state and processes actions.
///
/// A store is responsible for holding the current state, applying reducers
/// to update that state in response to dispatched actions, and invoking
/// triggers to produce any follow-up actions.
///
/// The store enforces a unidirectional flow:
/// actions are dispatched → reducers update state → triggers may emit
/// additional actions → repeat until no further actions remain.
///
/// Consumers interact with the store by dispatching actions and selecting
/// derived values from state via selectors. The store itself should not
/// contain business logic; it orchestrates reducers and triggers, which
/// define the system’s behavior.
pub struct Store<TState: 'static> {
    state: RwLock<Arc<TState>>,
}

impl<TState> Store<TState> {
    pub fn new(state: TState) -> Self {
        Self {
            state: RwLock::new(Arc::new(state)),
        }
    }

    pub fn select<'a, T: 'a>(
        &'a self,
        select: impl FnOnce(&'a TState) -> T + 'a,
    ) -> Accessor<'a, TState, T> {
        let state = self.state.read().clone();
        Accessor::new(state, select)
    }

    pub fn dispatcher(&self) -> Dispatcher<'_, TState> {
        Dispatcher::new(self.state.write())
    }
}

impl<TState> Default for Store<TState>
where
    TState: Default,
{
    fn default() -> Self {
        Self::new(TState::default())
    }
}

impl<TState> std::fmt::Debug for Store<TState>
where
    TState: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(std::any::type_name::<Self>())
            .field(&self.state)
            .finish()
    }
}
