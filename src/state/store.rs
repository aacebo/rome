use std::sync::RwLock;

use crate::state::{Action, ActionBuffer, Selector, Trigger};

/// Central coordinator that owns state and processes actions.
pub struct Store<TState: 'static> {
    state: RwLock<TState>,
    buffer: ActionBuffer<TState>,
    triggers: RwLock<Vec<Box<dyn Trigger<TState>>>>,
}

impl<TState: 'static> Store<TState> {
    pub fn new(value: TState) -> Self {
        Self {
            state: RwLock::new(value),
            buffer: ActionBuffer::with_capacity(1024),
            triggers: RwLock::new(vec![]),
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.buffer = ActionBuffer::with_capacity(capacity);
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
    /// buffer is full (see `ActionBuffer::push`).
    pub fn dispatch<TAction: Action<State = TState>>(&self, action: TAction) {
        self.buffer.push(action);
    }

    pub fn trigger<TTrigger: Trigger<TState>>(&self, trigger: TTrigger) {
        self.triggers.write().unwrap().push(Box::new(trigger));
    }

    /// Drain queued actions and apply them in order to the current state.
    ///
    /// Concurrent flushes serialize behind the `RwLock` write guard —
    /// correct but wasteful; callers should typically have a single flusher.
    pub fn flush(&self) {
        let drained = self.buffer.drain();

        if drained.is_empty() {
            return;
        }

        let mut state = self.state.write().unwrap();

        for action in &drained {
            action.reduce(&mut state);
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
