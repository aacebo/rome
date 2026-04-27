use std::{pin::Pin, sync::Arc};

pub struct Stateful<T> {
    value: Option<Arc<T>>,
    stream: Pin<Box<dyn futures::Stream<Item = T>>>,
}

impl<T> Stateful<T> {
    pub(super) fn new(stream: impl futures::Stream<Item = T> + 'static) -> Self {
        Self {
            value: None,
            stream: Box::pin(stream),
        }
    }
}

impl<T> futures::Stream for Stateful<T> {
    type Item = Arc<T>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.stream.as_mut().poll_next(cx) {
            std::task::Poll::Ready(Some(value)) => {
                let ptr = Arc::new(value);
                self.value = Some(ptr.clone());
                std::task::Poll::Ready(Some(ptr))
            }
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

impl<T> std::ops::Deref for Stateful<T> {
    type Target = Option<Arc<T>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
