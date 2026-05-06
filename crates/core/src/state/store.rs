use std::{any::TypeId, collections::HashMap, sync::RwLock};

use crate::state::{Selector, State, select::Select};

use super::{Action, Trigger, action::Next, trigger};

/// Central coordinator that owns state and processes actions.
pub struct Store<TState: 'static> {
    state: State<TState>,
    buffer: Next<TState>,
    triggers: RwLock<HashMap<TypeId, Vec<Box<dyn trigger::ErasedTrigger<TState>>>>>,
}

impl<TState: 'static> Store<TState> {
    pub fn new(state: TState) -> Self {
        Self {
            state: State::new(state),
            buffer: Next::with_capacity(1024),
            triggers: RwLock::new(HashMap::new()),
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.buffer = Next::with_capacity(capacity);
        self
    }

    pub fn select<T>(&self) -> Select<'_, TState, T>
    where
        T: Selector<TState>,
    {
        Select::from(self.state.as_ref())
    }

    /// Queue an action for application on the next `flush`. Blocks if the
    /// buffer is full (see `Next::push`).
    pub fn dispatch<TAction>(&self, action: TAction)
    where
        TAction: Action<State = TState>,
    {
        self.buffer.dispatch(action);
    }

    /// Register a new Trigger that will be executed for each dispatch of [`TAction`]
    pub fn trigger<TAction, T>(&self, trigger: T)
    where
        TAction: Action<State = TState>,
        T: Trigger<TAction>,
    {
        self.triggers
            .write()
            .unwrap()
            .entry(TypeId::of::<TAction>())
            .or_default()
            .push(Box::new(trigger::TriggerGuard::<TAction, T>::new(trigger)));
    }

    /// Drain queued actions and apply them in order to the current state.
    pub fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let triggers = self.triggers.read().unwrap();

        while let drained = self.buffer.drain()
            && !drained.is_empty()
        {
            for action in &drained {
                action.reduce(self.state.as_mut());

                if let Some(bucket) = triggers.get(&action.type_id()) {
                    for trigger in bucket {
                        trigger.execute_erased(self.state.as_mut(), action.as_ref(), &self.buffer);
                    }
                }
            }
        }
    }
}

impl<TState: Default + 'static> Default for Store<TState> {
    fn default() -> Self {
        Self::new(TState::default())
    }
}

impl<TState: 'static> Drop for Store<TState> {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            self.flush();
        }
    }
}
