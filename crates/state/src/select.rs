/// Used to access/extract a slice of
/// [`TState`] in a safe immutable way.
pub trait Selector<TState> {
    fn select(state: &TState) -> &Self;
}

pub struct Select<'a, TState, T>
where
    T: Selector<TState>,
{
    out: &'a T,

    __marker__: std::marker::PhantomData<TState>,
}

impl<'a, TState, T> From<&'a TState> for Select<'a, TState, T>
where
    T: Selector<TState>,
{
    fn from(state: &'a TState) -> Self {
        let out = T::select(state);

        Self {
            out,
            __marker__: std::marker::PhantomData,
        }
    }
}

impl<'a, TState, T> std::ops::Deref for Select<'a, TState, T>
where
    T: Selector<TState>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.out
    }
}
