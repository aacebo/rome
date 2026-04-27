pub struct SignalStream<T> {
    waker: futures::task::AtomicWaker,
    receiver: crossbeam::channel::Receiver<T>,
}

impl<T> SignalStream<T> {
    pub(super) fn new(
        waker: futures::task::AtomicWaker,
        receiver: crossbeam::channel::Receiver<T>,
    ) -> Self {
        Self { waker, receiver }
    }
}

impl<T> futures::Stream for SignalStream<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.receiver.try_recv() {
            Ok(value) => return std::task::Poll::Ready(Some(value)),
            Err(crossbeam::channel::TryRecvError::Disconnected) => {
                return std::task::Poll::Ready(None);
            }
            Err(crossbeam::channel::TryRecvError::Empty) => {}
        }

        self.waker.register(cx.waker());

        // Re-check after registering to avoid lost wakeups.
        match self.receiver.try_recv() {
            Ok(value) => std::task::Poll::Ready(Some(value)),
            Err(crossbeam::channel::TryRecvError::Disconnected) => std::task::Poll::Ready(None),
            Err(crossbeam::channel::TryRecvError::Empty) => std::task::Poll::Pending,
        }
    }
}
