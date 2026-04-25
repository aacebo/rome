use std::sync::{Arc, Mutex, Weak};

pub struct Reader<T> {
    id: u64,
    handle: Arc<ReaderRef<T>>,
    source: Weak<super::_Signal<T>>,
}

impl<T> Reader<T> {
    pub(super) fn new(id: u64, handle: Arc<ReaderRef<T>>, source: Weak<super::_Signal<T>>) -> Self {
        Self { id, handle, source }
    }

    pub fn get(&self) -> Option<Arc<T>> {
        Some(self.source.upgrade()?.get())
    }
}

impl<T> Drop for Reader<T> {
    fn drop(&mut self) {
        if let Some(source) = self.source.upgrade() {
            source.remove(self.id);
        }
    }
}

impl<T> futures::Stream for Reader<T> {
    type Item = Arc<T>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if let Some(v) = self.handle.pending.lock().unwrap().take() {
            return std::task::Poll::Ready(Some(v));
        }

        if self.source.strong_count() == 0 {
            return std::task::Poll::Ready(None);
        }

        self.handle.waker.register(cx.waker());

        // Re-check after register: a Signal drop racing with register
        // would otherwise leave the task parked forever.
        if self.source.strong_count() == 0 {
            return std::task::Poll::Ready(None);
        }

        std::task::Poll::Pending
    }
}

pub(super) struct ReaderRef<T> {
    waker: futures::task::AtomicWaker,
    pending: Mutex<Option<Arc<T>>>,
}

impl<T> ReaderRef<T> {
    pub fn new() -> Self {
        Self {
            waker: futures::task::AtomicWaker::new(),
            pending: Mutex::new(None),
        }
    }

    pub fn next(&self, value: Arc<T>) {
        *self.pending.lock().unwrap() = Some(value);
        self.waker.wake();
    }

    pub fn wake(&self) {
        self.waker.wake();
    }
}
