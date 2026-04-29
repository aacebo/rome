use std::{any::TypeId, collections::HashMap, sync::RwLock};

use crate::{Action, Next, Selector, Trigger, trigger};

/// Central coordinator that owns state and processes actions.
pub struct Store<TState: 'static> {
    state: RwLock<TState>,
    buffer: Next<TState>,
    triggers: RwLock<HashMap<TypeId, Vec<Box<dyn trigger::ErasedTrigger<TState>>>>>,
}

impl<TState: 'static> Store<TState> {
    pub fn new(value: TState) -> Self {
        Self {
            state: RwLock::new(value),
            buffer: Next::with_capacity(1024),
            triggers: RwLock::new(HashMap::new()),
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.buffer = Next::with_capacity(capacity);
        self
    }

    pub fn select<T, F>(&self, project: F) -> Selector<'_, TState, T>
    where
        F: Fn(&TState) -> T + Send + Sync + 'static,
        T: 'static,
    {
        Selector::map(self.state.read().unwrap(), project)
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
    ///
    /// Concurrent flushes serialize behind the `RwLock` write guard —
    /// correct but wasteful; callers should typically have a single flusher.
    pub fn flush(&self) {
        if self.buffer.is_empty() {
            return;
        }

        let mut state = self.state.write().unwrap();
        let triggers = self.triggers.read().unwrap();

        while let drained = self.buffer.drain()
            && !drained.is_empty()
        {
            for action in &drained {
                action.reduce(&mut state);

                if let Some(bucket) = triggers.get(&action.type_id()) {
                    for trigger in bucket {
                        trigger.execute_erased(&*state, action.as_ref(), &self.buffer);
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
