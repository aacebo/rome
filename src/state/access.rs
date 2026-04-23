use std::{cell::LazyCell, sync::Arc};

pub struct Accessor<'a, TState, T> {
    _state: Arc<TState>,
    cell: LazyCell<T, Box<dyn FnOnce() -> T + 'a>>,
}

impl<'a, TState: 'static, T: 'a> Accessor<'a, TState, T> {
    pub fn new<TSelect>(state: Arc<TState>, select: TSelect) -> Self
    where
        TSelect: FnOnce(&'a TState) -> T + 'a,
    {
        // SAFETY: `state` is moved into `_state` below. Arc's heap allocation
        // is pinned (Arc never moves its payload), so `&*state` remains valid
        // for as long as the Accessor (and therefore `_state`) is alive. The
        // phantom lifetime 'a represents that duration.
        let state_ref: &'a TState = unsafe { &*Arc::as_ptr(&state) };

        Self {
            _state: state,
            cell: LazyCell::new(Box::new(move || select(state_ref))),
        }
    }
}

impl<TState, T> std::ops::Deref for Accessor<'_, TState, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}

impl<TState, T: PartialEq> PartialEq<T> for Accessor<'_, TState, T> {
    fn eq(&self, other: &T) -> bool {
        &**self == other
    }
}

impl<TState, T: std::fmt::Debug> std::fmt::Debug for Accessor<'_, TState, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &**self)
    }
}
