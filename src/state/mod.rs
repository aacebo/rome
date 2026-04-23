mod store;

pub use store::*;

/// Represents an event that describes something that occurred in the system.
pub trait Action: Send + Sync + 'static {
    type State;

    fn name(&self) -> &'static str;
    fn reduce(self, state: &mut Self::State);
}

/// Projects a value from store state without mutating it.
///
/// A selector is a read-only query over state used to retrieve a derived
/// value, slice, or view of the current state. Selectors should be free of
/// side triggers and should not depend on external mutable state.
///
/// In most cases, selectors are small pure projections such as reading a
/// field, computing a count, or transforming part of the state into a more
/// convenient shape for consumers.
pub trait Selector<TState, TOut: ?Sized> = for<'a> Fn(&'a TState) -> &'a TOut;

/// Reacts to an action and state transition by performing follow-up work.
///
/// A `Trigger` is the side-effecting counterpart to a reducer. Whereas reducers
/// synchronously derive new state from an action, triggers observe the current
/// state and incoming action and may produce further actions by dispatching them
/// through the provided [`Dispatcher`].
pub trait Trigger<TAction: Action> {
    fn trigger(
        &self,
        state: &TAction::State,
        action: &TAction,
    ) -> impl Future<Output = impl futures::Stream<Item = TAction>>;
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    struct UserState {
        pub name: String,
    }

    enum UserAction {
        Rename(String),
    }

    impl Action for UserAction {
        type State = UserState;

        fn name(&self) -> &'static str {
            "user"
        }

        fn reduce(self, state: &mut Self::State) {
            match self {
                Self::Rename(v) => {
                    state.name = v;
                }
            }
        }
    }

    #[test]
    fn user_name_selected() {
        let store = Store::new(UserState {
            name: "test user".to_string(),
        });

        let name = store.select_as(|s: &UserState| &s.name);

        assert_eq!(name.deref(), "test user");
        assert_eq!(store.select(|s| s.name.len()), 9);

        store
            .dispatcher()
            .dispatch(UserAction::Rename("hello world".to_string()));

        assert_eq!(name.deref(), "hello world");
    }
}
