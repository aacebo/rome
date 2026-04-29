use crate::{Action, Next};

pub trait Trigger<TAction: Action>: Send + Sync + 'static {
    fn execute(&self, state: &TAction::State, action: &TAction, next: &Next<TAction::State>);
}

impl<TAction, F> Trigger<TAction> for F
where
    TAction: Action,
    F: Fn(&TAction::State, &TAction, &Next<TAction::State>) + Send + Sync + 'static,
{
    fn execute(&self, state: &TAction::State, action: &TAction, next: &Next<TAction::State>) {
        (self)(state, action, next);
    }
}

pub(super) trait ErasedTrigger<TState>: Send + Sync + 'static {
    fn execute_erased(
        &self,
        state: &TState,
        action: &dyn Action<State = TState>,
        next: &Next<TState>,
    );
}

pub(super) struct TriggerGuard<TAction, T>
where
    TAction: Action,
    T: Trigger<TAction>,
{
    inner: T,

    __marker__: std::marker::PhantomData<fn(TAction)>,
}

impl<TAction, T> TriggerGuard<TAction, T>
where
    TAction: Action,
    T: Trigger<TAction>,
{
    pub(super) fn new(inner: T) -> Self {
        Self {
            inner,
            __marker__: std::marker::PhantomData,
        }
    }
}

impl<TAction, T> ErasedTrigger<TAction::State> for TriggerGuard<TAction, T>
where
    TAction: Action,
    T: Trigger<TAction>,
{
    fn execute_erased(
        &self,
        state: &TAction::State,
        action: &dyn Action<State = TAction::State>,
        next: &Next<TAction::State>,
    ) {
        if let Some(action) = (action as &dyn std::any::Any).downcast_ref::<TAction>() {
            self.inner.execute(state, action, next);
        }
    }
}
