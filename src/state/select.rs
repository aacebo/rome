use std::{cell::OnceCell, sync::RwLockReadGuard};

/// Borrowed projection over the `Store`'s state, returned by
/// [`Store::select`].
///
/// Holds an `RwLockReadGuard` and lazily applies the projection on first
/// access via `AsRef`/`Deref`; the result is cached in a `OnceCell` for
/// repeated reads. The guard is held for the `Selector`'s lifetime, so
/// callers should drop it before dispatching actions that need to flush
/// (which would block waiting for the write lock). `PartialEq` is
/// implemented so callers can compare directly against `T` literals.
pub struct Selector<'a, TState, TOut> {
    state: RwLockReadGuard<'a, TState>,
    output: OnceCell<TOut>,
    project: Box<dyn Fn(&TState) -> TOut>,
}

impl<'a, TState, TOut> Selector<'a, TState, TOut> {
    pub fn new(
        state: RwLockReadGuard<'a, TState>,
        project: impl Fn(&TState) -> TOut + 'static,
    ) -> Self {
        Self {
            state,
            output: OnceCell::new(),
            project: Box::new(project),
        }
    }
}

impl<'a, TState, TOut, TOther> PartialEq<TOther> for Selector<'a, TState, TOut>
where
    TOut: PartialEq<TOther>,
{
    fn eq(&self, other: &TOther) -> bool {
        self.as_ref().eq(other)
    }
}

impl<'a, TState, TOut> std::fmt::Debug for Selector<'a, TState, TOut>
where
    TOut: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, TState, TOut> std::fmt::Display for Selector<'a, TState, TOut>
where
    TOut: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, TState, TOut> std::ops::Deref for Selector<'a, TState, TOut> {
    type Target = TOut;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, TState, TOut> AsRef<TOut> for Selector<'a, TState, TOut> {
    fn as_ref(&self) -> &TOut {
        self.output.get_or_init(|| (self.project)(&self.state))
    }
}
