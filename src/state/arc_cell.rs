use std::sync::{Arc, nonpoison::Mutex};

/// A cell holding an `Arc<T>` that can be atomically swapped.
///
/// Reads take a brief lock (just long enough to clone the Arc) then release;
/// all subsequent access to the data is lock-free through the cloned Arc.
/// Writes take a brief lock to install a new Arc. Previously-loaded snapshots
/// remain valid and unchanged — the key property behind the copy-on-write
/// dispatch pattern used by `Store`.
pub struct ArcCell<T>(Mutex<Arc<T>>);

impl<T> ArcCell<T> {
    pub fn new(value: T) -> Self {
        Self(Mutex::new(Arc::new(value)))
    }

    pub fn load(&self) -> Arc<T> {
        self.0.lock().clone()
    }

    pub fn store(&self, value: Arc<T>) {
        *self.0.lock() = value;
    }

    /// Atomically swap to `new` iff the currently-stored Arc is the same
    /// allocation as `expected` (pointer equality via `Arc::ptr_eq`).
    /// Returns `Ok(())` on success, or `Err(current)` giving the caller a
    /// fresh snapshot to retry with.
    pub fn compare_and_swap(&self, expected: &Arc<T>, new: Arc<T>) -> Result<(), Arc<T>> {
        let mut guard = self.0.lock();

        if Arc::ptr_eq(&*guard, expected) {
            *guard = new;
            Ok(())
        } else {
            Err(guard.clone())
        }
    }

    /// Read-copy-update. Repeatedly builds a new value from the current one
    /// and tries to install it atomically; retries if another writer raced us.
    /// The closure may be invoked multiple times under contention, so it must
    /// be pure (no observable side effects). Returns the installed Arc.
    pub fn rcu(&self, mut f: impl FnMut(&T) -> T) -> Arc<T> {
        let mut current = self.load();

        loop {
            let next = Arc::new(f(&*current));

            match self.compare_and_swap(&current, next.clone()) {
                Ok(()) => return next,
                Err(fresh) => current = fresh,
            }
        }
    }
}
