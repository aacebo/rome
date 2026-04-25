use super::Operator;

pub fn map<F>(f: F) -> Map<F> {
    Map { f }
}

pub struct Map<F> {
    f: F,
}

impl<S, F, U> Operator<S> for Map<F>
where
    S: futures::Stream,
    F: FnMut(S::Item) -> U,
{
    type Output = futures::stream::Map<S, F>;

    fn apply(self, stream: S) -> Self::Output {
        futures::StreamExt::map(stream, self.f)
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    use super::super::Pipe;
    use super::*;
    use crate::state::source::{Source, Stream};

    fn poll_once<S: futures::Stream + Unpin>(s: &mut S) -> Poll<Option<S::Item>> {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(s).poll_next(&mut cx)
    }

    #[test]
    fn transforms_values() {
        let source = Source::new(1u32);
        let mut piped = source.stream().pipe(map(|v: Arc<u32>| *v * 2));

        source.emit(5);
        match poll_once(&mut piped) {
            Poll::Ready(Some(v)) => assert_eq!(v, 10),
            other => panic!("expected Ready(Some(10)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn chains() {
        let source = Source::new(0u32);
        let mut piped = source
            .stream()
            .pipe(map(|v: Arc<u32>| *v + 1))
            .pipe(map(|v: u32| v * 10));

        source.emit(4);
        match poll_once(&mut piped) {
            Poll::Ready(Some(v)) => assert_eq!(v, 50),
            other => panic!("expected Ready(Some(50)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn terminates_on_source_drop() {
        let source = Source::new(0u32);
        let mut piped = source.stream().pipe(map(|v: Arc<u32>| *v));

        drop(source);

        match poll_once(&mut piped) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn is_send_static() {
        fn assert_send_static<T: Send + 'static>() {}
        assert_send_static::<futures::stream::Map<Stream<u32>, fn(Arc<u32>) -> u32>>();
    }
}
