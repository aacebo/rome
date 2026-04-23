use std::sync::{
    Arc,
    nonpoison::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// Represents an event that describes something that occurred in the system.
pub trait Action: Send + Sync + 'static {
    type State;

    fn name(&self) -> &'static str;
    fn reduce(self, state: &mut Self::State);
}

/// Projects a value from store state without mutating it.
///
/// A selector is a read-only query over state used to retrieve a derived
/// value, slice, or view of the current state. Selectors should be free of
/// side triggers and should not depend on external mutable state.
///
/// In most cases, selectors are small pure projections such as reading a
/// field, computing a count, or transforming part of the state into a more
/// convenient shape for consumers.
pub trait Selector<TState, TOut: ?Sized> = for<'a> Fn(&'a TState) -> &'a TOut;

/// Reacts to an action and state transition by performing follow-up work.
///
/// A `Trigger` is the side-effecting counterpart to a reducer. Whereas reducers
/// synchronously derive new state from an action, triggers observe the current
/// state and incoming action and may produce further actions by dispatching them
/// through the provided [`Dispatcher`].
pub trait Trigger<TAction: Action> {
    fn trigger(
        &self,
        state: &TAction::State,
        action: &TAction,
    ) -> impl Future<Output = impl futures::Stream<Item = TAction>>;
}

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

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    struct UserState {
        pub name: String,
    }

    enum UserAction {
        Rename(String),
    }

    impl Action for UserAction {
        type State = UserState;

        fn name(&self) -> &'static str {
            "user"
        }

        fn reduce(self, state: &mut Self::State) {
            match self {
                Self::Rename(v) => {
                    state.name = v;
                }
            }
        }
    }

    #[test]
    fn user_name_selected() {
        let store = Store::new(UserState {
            name: "test user".to_string(),
        });

        let name = store.select_as(|s: &UserState| &s.name);

        assert_eq!(name.deref(), "test user");
        assert_eq!(store.select(|s| s.name.len()), 9);

        store
            .dispatcher()
            .dispatch(UserAction::Rename("hello world".to_string()));

        assert_eq!(name.deref(), "hello world");
    }
}
