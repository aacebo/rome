mod map;

pub use map::*;

pub trait Operator<S> {
    type Output;

    fn apply(self, input: S) -> Self::Output;
}
