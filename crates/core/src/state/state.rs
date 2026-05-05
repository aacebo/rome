use std::sync::atomic::{AtomicU64, Ordering};

pub struct State<T> {
    ver: AtomicU64,
    inner: T,
}

impl<T> State<T> {
    pub fn new(inner: T) -> Self {
        Self {
            ver: AtomicU64::new(0),
            inner,
        }
    }

    pub fn version(&self) -> u64 {
        self.ver.load(Ordering::Acquire)
    }

    pub fn transact(&mut self) -> Tx<'_, T> {
        Tx { state: self }
    }
}

impl<T> AsRef<T> for State<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> std::ops::Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Tx<'a, T> {
    state: &'a mut State<T>,
}

impl<'a, T> AsRef<T> for Tx<'a, T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<'a, T> AsMut<T> for Tx<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<'a, T> std::ops::Deref for Tx<'a, T> {
    type Target = State<T>;

    fn deref(&self) -> &Self::Target {
        self.state
    }
}

impl<'a, T> std::ops::DerefMut for Tx<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.state
    }
}

impl<'a, T> Drop for Tx<'a, T> {
    fn drop(&mut self) {
        self.state.ver.fetch_add(1, Ordering::Relaxed);
    }
}
