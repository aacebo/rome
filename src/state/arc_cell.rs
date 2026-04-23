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
}

impl<T: Clone> ArcCell<T> {
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.0.lock();
        f(Arc::make_mut(&mut *guard))
    }
}
