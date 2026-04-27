use std::sync::nonpoison::Mutex;

use crate::state::{Action, ActionBuffer, Signal};

/// Central coordinator that owns state and processes actions.
///
/// Dispatches are queued into a bounded lock-free buffer; state is advanced
/// in batches by `flush`, which clones the current state once, runs every
/// queued action's reducer against the clone, and emits the new state via
/// the inner `Signal`.
///
/// Ordering note: `ArrayQueue` is FIFO across all producers, but the
/// interleaving of pushes from different threads is determined by atomic
/// arrival order. Non-commutative reducers may produce different final
/// states across runs under contention.
pub struct Store<TState: Clone + 'static> {
    state: Signal<TState>,
    buffer: ActionBuffer<TState>,
    flush_lock: Mutex<()>,
}

impl<TState: Clone + 'static> Store<TState> {
    pub fn new(init: TState) -> Self {
        Self {
            state: Signal::new(init),
            buffer: ActionBuffer::with_capacity(1024),
            flush_lock: Mutex::new(()),
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.buffer = ActionBuffer::with_capacity(capacity);
        self
    }

    // pub fn select<T>(
    //     &self,
    //     f: impl Fn(&TState) -> T + Send + Sync + 'static,
    // ) -> signal::Select<TState, T>
    // where
    //     TState: Send + Sync,
    //     T: Send + 'static,
    // {
    //     signal::Select::new(self.state.clone(), f)
    // }

    /// Queue an action for application on the next `flush`. Blocks if the
    /// buffer is full (see `ActionBuffer::push`).
    pub fn dispatch<TAction: Action<State = TState>>(&self, action: TAction) {
        self.buffer.push(action);
    }

    /// Drain queued actions and apply them in order to the current state in
    /// place. `Signal::with_mut` uses `Arc::make_mut`, so the state clone is
    /// elided when no subscriber holds an outstanding `Arc<TState>`. After
    /// all actions are applied, subscribers are notified with one
    /// `Arc<TState>`.
    ///
    /// Concurrent flushes serialize behind an internal lock — correct but
    /// wasteful; callers should typically have a single flusher.
    pub fn flush(&self) {
        let _guard = self.flush_lock.lock();
        let drained = self.buffer.drain();

        if drained.is_empty() {
            return;
        }

        self.state.with_mut(|state| {
            for action in &drained {
                action.reduce(state);
            }
        });
    }
}

impl<TState: Clone + Default + 'static> Default for Store<TState> {
    fn default() -> Self {
        Self::new(TState::default())
    }
}

impl<TState: Clone + 'static> Drop for Store<TState> {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            self.flush();
        }
    }
}
