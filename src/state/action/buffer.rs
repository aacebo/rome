use crossbeam::queue::ArrayQueue;

use crate::state::Action;

/// A bounded, lock-free, multi-producer queue of pending actions.
///
/// `push` is called from any thread and blocks (spin-then-yield) if the
/// queue is full; the flusher drains via `drain_into`. Capacity is fixed
/// at construction — callers must size it so that backpressure is rare,
/// or producers will stall waiting for the flusher.
pub struct ActionBuffer<TState: 'static> {
    pending: ArrayQueue<Box<dyn Action<State = TState>>>,
}

impl<TState: 'static> ActionBuffer<TState> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            pending: ArrayQueue::new(cap.max(1)),
        }
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.pending.capacity()
    }

    /// Enqueue an action. Blocks (spin, then yield) until space is available.
    pub fn push<TAction: Action<State = TState>>(&self, action: TAction) -> &Self {
        let mut boxed: Box<dyn Action<State = TState>> = Box::new(action);
        let mut spins = 0u32;

        loop {
            match self.pending.push(boxed) {
                Ok(()) => break,
                Err(returned) => {
                    boxed = returned;

                    if spins < 8 {
                        std::hint::spin_loop();
                        spins += 1;
                    } else {
                        std::thread::yield_now();
                    }
                }
            }
        }

        self
    }

    /// Drain all currently-queued actions into `sink` and return the count
    /// drained. Non-blocking: pushes arriving after this returns are picked
    /// up on the next drain.
    pub fn drain(&self) -> Vec<Box<dyn Action<State = TState>>> {
        let mut actions = Vec::with_capacity(self.pending.len());

        while let Some(action) = self.pending.pop() {
            actions.push(action);
        }

        actions
    }
}
