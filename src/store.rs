use std::sync::{Arc, RwLock};

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
pub trait Selector<TState> {
    type Out;

    fn select(&self, state: &TState) -> Self::Out;
}

impl<TState, TOut, T> Selector<TState> for T
where
    Self: Fn(&TState) -> TOut,
{
    type Out = TOut;

    fn select(&self, state: &TState) -> Self::Out {
        self(state)
    }
}

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

    pub fn select<TSelector>(&self, selector: &TSelector) -> TSelector::Out
    where
        TSelector: Selector<TState>,
    {
        let state = self.state.read().unwrap();
        selector.select(&state)
    }

    pub fn dispatch<TAction>(&self, action: TAction)
    where
        TAction: Action<State = TState>,
    {
        let mut state = self.state.write().unwrap();
        action.reduce(&mut state);
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
