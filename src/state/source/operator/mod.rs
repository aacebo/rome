mod map;

pub use map::*;

pub trait Operator<S> {
    type Output: futures::Stream;

    fn apply(self, stream: S) -> Self::Output;
}

pub trait Pipe: futures::Stream + Sized {
    fn pipe<O>(self, op: O) -> O::Output
    where
        O: Operator<Self>,
    {
        op.apply(self)
    }
}

impl<S: futures::Stream + Sized> Pipe for S {}
