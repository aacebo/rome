mod access;
mod arc_cell;
mod store;

pub use access::*;
pub use arc_cell::*;
pub use store::*;

/// Represents an event that describes something that occurred in the system.
pub trait Action: Send + Sync + 'static {
    type State;

    fn name(&self) -> &'static str;
    fn reduce(self, state: &mut Self::State);
}

/// Reacts to an action and state transition by performing follow-up work.
///
/// A `Trigger` is the side-effecting counterpart to a reducer. Whereas reducers
/// synchronously derive new state from an action, triggers observe the current
/// state and incoming action and may produce further actions by dispatching them
/// back into the store.
pub trait Trigger<TAction: Action> {
    fn trigger(
        &self,
        state: &TAction::State,
        action: &TAction,
    ) -> impl Future<Output = impl futures::Stream<Item = TAction>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct UserState {
        pub name: String,
    }

    #[derive(Clone)]
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

        let name = store.select(|s| s.name.as_str());

        assert_eq!(name, "test user");
        assert_eq!(name.len(), 9);

        store.dispatch(UserAction::Rename("hello world".to_string()));

        assert_eq!(name, "test user");

        assert_eq!(store.select(|s| s.name.as_str()), "hello world");
    }

    #[test]
    fn concurrent_dispatch_no_lost_updates() {
        use std::sync::Arc;

        #[derive(Clone)]
        struct Counter {
            n: u32,
        }

        #[derive(Clone)]
        struct Bump;

        impl Action for Bump {
            type State = Counter;

            fn name(&self) -> &'static str {
                "bump"
            }

            fn reduce(self, state: &mut Counter) {
                state.n += 1;
            }
        }

        let store = Arc::new(Store::new(Counter { n: 0 }));

        std::thread::scope(|scope| {
            for _ in 0..4 {
                let s = store.clone();
                scope.spawn(move || {
                    for _ in 0..1000 {
                        s.dispatch(Bump);
                    }
                });
            }
        });

        assert_eq!(store.select(|c| c.n), 4000);
    }
}
