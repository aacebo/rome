use crate::state::{Accessor, Action, ArcCell};

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
    state: ArcCell<TState>,
}

impl<TState: 'static> Store<TState> {
    pub fn new(init: TState) -> Self {
        Self {
            state: ArcCell::new(init),
        }
    }

    pub fn select<'a, T: 'a>(
        &'a self,
        select: impl FnOnce(&'a TState) -> T + 'a,
    ) -> Accessor<'a, TState, T> {
        Accessor::new(self.state.load(), select)
    }
}

impl<TState: Clone + 'static> Store<TState> {
    /// Dispatch an action, applying its reducer under a compare-and-swap loop.
    ///
    /// The reducer may run more than once under concurrent dispatches — it must
    /// be pure. Clones happen outside the lock; the critical section is a
    /// pointer compare + swap.
    pub fn dispatch<TAction: Action<State = TState> + Clone>(&self, action: TAction) {
        let mut current = self.state.load();

        loop {
            let mut next = (*current).clone();
            action.clone().reduce(&mut next);

            match self
                .state
                .compare_and_swap(&current, std::sync::Arc::new(next))
            {
                Ok(()) => return,
                Err(fresh) => current = fresh,
            }
        }
    }
}

impl<TState: Default + 'static> Default for Store<TState> {
    fn default() -> Self {
        Self::new(TState::default())
    }
}

impl<TState: std::fmt::Debug + 'static> std::fmt::Debug for Store<TState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(std::any::type_name::<Self>())
            .field(&*self.state.load())
            .finish()
    }
}
