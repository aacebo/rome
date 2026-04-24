use std::sync::{Arc, nonpoison::Mutex};

use crate::state::{Accessor, Action, ActionBuffer, ArcCell};

/// Central coordinator that owns state and processes actions.
///
/// Dispatches are queued into a bounded lock-free buffer; state is advanced
/// in batches by `flush`, which clones the current state once, runs every
/// queued action's reducer against the clone, installs the new state via
/// `ArcCell`, and appends the applied actions to an unbounded history log.
///
/// Ordering note: `ArrayQueue` is FIFO across all producers, but the
/// interleaving of pushes from different threads is determined by atomic
/// arrival order. Non-commutative reducers may produce different final
/// states across runs under contention.
pub struct Store<TState: Clone + 'static> {
    state: ArcCell<TState>,
    buffer: ActionBuffer<TState>,
    history: Mutex<Vec<Box<dyn Action<State = TState>>>>,
    flush_lock: Mutex<()>,
}

impl<TState: Clone + 'static> Store<TState> {
    pub fn new(init: TState) -> Self {
        Self {
            state: ArcCell::new(init),
            buffer: ActionBuffer::with_capacity(1024),
            history: Mutex::new(Vec::new()),
            flush_lock: Mutex::new(()),
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.buffer = ActionBuffer::with_capacity(capacity);
        self
    }

    pub fn select<'a, T: 'a>(
        &'a self,
        select: impl FnOnce(&'a TState) -> T + 'a,
    ) -> Accessor<'a, TState, T> {
        Accessor::new(self.state.load(), select)
    }

    /// Queue an action for application on the next `flush`. Blocks if the
    /// buffer is full (see `ActionBuffer::push`).
    pub fn dispatch<TAction: Action<State = TState>>(&self, action: TAction) {
        self.buffer.push(action);
    }

    /// Drain queued actions, apply them in order to a fresh clone of state,
    /// install the new state, and append the actions to the history log.
    ///
    /// Concurrent flushes serialize behind an internal lock — correct but
    /// wasteful; callers should typically have a single flusher.
    pub fn flush(&self) {
        let _guard = self.flush_lock.lock();
        let mut drained: Vec<Box<dyn Action<State = TState>>> = Vec::new();

        self.buffer.drain_into(&mut drained);

        if drained.is_empty() {
            return;
        }

        let current = self.state.load();
        let mut next = (*current).clone();

        for action in &drained {
            action.reduce(&mut next);
        }

        self.state.store(Arc::new(next));

        let mut history = self.history.lock();
        history.extend(drained);
    }

    pub fn history_len(&self) -> usize {
        self.history.lock().len()
    }

    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn buffer_is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<TState: Clone + Default + 'static> Store<TState> {
    /// Rebuild a `TState` by replaying history entries `[from..]` against
    /// `TState::default()`. Used for in-memory undo/redo and time-travel
    /// debugging. Requires deterministic reducers.
    pub fn replay_from(&self, from: usize) -> TState {
        let history = self.history.lock();
        let mut state = TState::default();

        for action in &history[from..] {
            action.reduce(&mut state);
        }

        state
    }
}

impl<TState: Clone + Default + 'static> Default for Store<TState> {
    fn default() -> Self {
        Self::new(TState::default())
    }
}

impl<TState: Clone + std::fmt::Debug + 'static> std::fmt::Debug for Store<TState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("state", &*self.state.load())
            .field("pending", &self.buffer.len())
            .field("history_len", &self.history.lock().len())
            .finish()
    }
}

impl<TState: Clone + 'static> Drop for Store<TState> {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            self.flush();
        }
    }
}
