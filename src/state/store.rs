use std::cell::OnceCell;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use crate::state::{Action, ActionBuffer};

/// Central coordinator that owns state and processes actions.
///
/// Dispatches are queued into a bounded lock-free buffer; `flush` takes the
/// `RwLock` write guard once and applies every queued reducer in order
/// against the live state.
///
/// Ordering note: `ArrayQueue` is FIFO across all producers, but the
/// interleaving of pushes from different threads is determined by atomic
/// arrival order. Non-commutative reducers may produce different final
/// states across runs under contention.
pub struct Store<TState: Clone + 'static> {
    state: RwLock<TState>,
    buffer: ActionBuffer<TState>,
}

impl<TState: Clone + 'static> Store<TState> {
    pub fn new(value: TState) -> Self {
        Self {
            state: RwLock::new(value),
            buffer: ActionBuffer::with_capacity(1024),
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
        Selector::new(self.state.read().unwrap(), project)
    }

    /// Queue an action for application on the next `flush`. Blocks if the
    /// buffer is full (see `ActionBuffer::push`).
    pub fn dispatch<TAction: Action<State = TState>>(&self, action: TAction) {
        self.buffer.push(action);
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

/// Borrowed projection over the `Store`'s state, returned by
/// [`Store::select`].
///
/// Holds an `RwLockReadGuard` and lazily applies the projection on first
/// access via `AsRef`/`Deref`; the result is cached in a `OnceCell` for
/// repeated reads. The guard is held for the `Selector`'s lifetime, so
/// callers should drop it before dispatching actions that need to flush
/// (which would block waiting for the write lock). `PartialEq` is
/// implemented so callers can compare directly against `T` literals.
pub struct Selector<'a, TState, TOut> {
    state: RwLockReadGuard<'a, TState>,
    output: OnceCell<TOut>,
    project: Box<dyn Fn(&TState) -> TOut>,
}

impl<'a, TState, TOut> Selector<'a, TState, TOut> {
    pub fn new(
        state: RwLockReadGuard<'a, TState>,
        project: impl Fn(&TState) -> TOut + 'static,
    ) -> Self {
        Self {
            state,
            output: OnceCell::new(),
            project: Box::new(project),
        }
    }
}

impl<'a, TState, TOut, TOther> PartialEq<TOther> for Selector<'a, TState, TOut>
where
    TOut: PartialEq<TOther>,
{
    fn eq(&self, other: &TOther) -> bool {
        self.as_ref().eq(other)
    }
}

impl<'a, TState, TOut> std::fmt::Debug for Selector<'a, TState, TOut>
where
    TOut: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, TState, TOut> std::fmt::Display for Selector<'a, TState, TOut>
where
    TOut: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, TState, TOut> std::ops::Deref for Selector<'a, TState, TOut> {
    type Target = TOut;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, TState, TOut> AsRef<TOut> for Selector<'a, TState, TOut> {
    fn as_ref(&self) -> &TOut {
        self.output.get_or_init(|| (self.project)(&self.state))
    }
}
