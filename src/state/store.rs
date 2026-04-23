use std::sync::{Arc, nonpoison::*};

use crate::state::{Action, Selector};

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
pub struct Store<TState> {
    state: Arc<RwLock<TState>>,
}

impl<TState> Store<TState> {
    pub fn new(state: TState) -> Self {
        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    pub fn select<R>(&self, selector: impl FnOnce(&TState) -> R) -> R {
        selector(&self.state.read())
    }

    pub fn select_as<TOut, TSelector>(&self, selector: TSelector) -> MappedRwLockReadGuard<'_, TOut>
    where
        TOut: ?Sized,
        TSelector: Selector<TState, TOut>,
    {
        RwLockReadGuard::map(self.state.read(), selector)
    }

    pub fn dispatcher(&self) -> Dispatcher<'_, TState> {
        Dispatcher(self.state.write())
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

pub struct Dispatcher<'a, TState>(RwLockWriteGuard<'a, TState>);

impl<'a, TState> Dispatcher<'a, TState> {
    pub fn dispatch<TAction>(&mut self, action: TAction)
    where
        TAction: Action<State = TState>,
    {
        action.reduce(&mut self.0);
    }
}
