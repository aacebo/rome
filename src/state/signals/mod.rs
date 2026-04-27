pub mod adapters;
mod compute;
mod stream;

use std::time::Duration;

pub use compute::*;
pub use stream::*;

pub struct Signal<T> {
    sender: crossbeam::channel::Sender<T>,
    receiver: crossbeam::channel::Receiver<T>,

    __marker__: std::marker::PhantomData<T>,
}

impl<T> Signal<T> {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam::channel::bounded(10);

        Self {
            sender,
            receiver,

            __marker__: std::marker::PhantomData,
        }
    }

    pub fn stream(&self) -> SignalStream<T> {
        SignalStream::new(futures::task::AtomicWaker::new(), self.receiver.clone())
    }

    pub fn next(&mut self, value: T) {
        self.sender
            .send_timeout(value, Duration::from_secs(1))
            .unwrap()
    }
}
