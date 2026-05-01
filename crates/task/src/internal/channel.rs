pub(crate) struct Channel<T>
where
    T: Send + 'static,
{
    sender: crossbeam::channel::Sender<T>,
    receiver: crossbeam::channel::Receiver<T>,
}

impl<T> Channel<T>
where
    T: Send + 'static,
{
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();

        Self { sender, receiver }
    }

    pub fn sender(&self) -> &crossbeam::channel::Sender<T> {
        &self.sender
    }

    pub fn receiver(&self) -> &crossbeam::channel::Receiver<T> {
        &self.receiver
    }
}
