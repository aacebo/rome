mod access;
mod buffer;
mod source;
mod store;

pub use access::*;
pub use buffer::*;
pub use source::*;
pub use store::*;

/// Represents an event that describes something that occurred in the system.
///
/// Reducers take `&self` so the boxed action can be moved into the replay log
/// after being applied. Reducers must be pure — they may run during replay
/// from any prior state.
pub trait Action: std::fmt::Debug + Send + Sync + 'static {
    type State;

    fn name(&self) -> &'static str;
    fn reduce(&self, state: &mut Self::State);
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

    #[derive(Clone, Default, Debug, PartialEq)]
    struct UserState {
        pub name: String,
    }

    #[derive(Debug)]
    enum UserAction {
        Rename(String),
    }

    impl Action for UserAction {
        type State = UserState;

        fn name(&self) -> &'static str {
            "user"
        }

        fn reduce(&self, state: &mut Self::State) {
            match self {
                Self::Rename(v) => {
                    state.name = v.clone();
                }
            }
        }
    }

    #[derive(Clone, Default, Debug, PartialEq)]
    struct Counter {
        n: u32,
    }

    #[derive(Debug)]
    struct Bump;

    impl Action for Bump {
        type State = Counter;

        fn name(&self) -> &'static str {
            "bump"
        }

        fn reduce(&self, state: &mut Counter) {
            state.n += 1;
        }
    }

    #[test]
    fn dispatch_is_lazy_until_flush() {
        let store = Store::new(UserState {
            name: "test user".to_string(),
        });

        store.dispatch(UserAction::Rename("hello world".to_string()));
        assert_eq!(store.select(|s| s.name.as_str()), "test user");

        store.flush();
        assert_eq!(store.select(|s| s.name.as_str()), "hello world");
    }

    #[test]
    fn flush_applies_dispatched_actions_in_order() {
        let store = Store::new(UserState::default());

        store.dispatch(UserAction::Rename("a".to_string()));
        store.dispatch(UserAction::Rename("b".to_string()));
        store.dispatch(UserAction::Rename("c".to_string()));
        store.flush();

        assert_eq!(store.select(|s| s.name.as_str()), "c");
    }

    #[test]
    fn multi_producer_push() {
        use std::sync::Arc;

        // 4 producers * 1000 pushes = 4000 actions, exceeds the default
        // 1024 capacity, so a consumer must flush concurrently or producers
        // will block forever on a full queue.
        let store = Arc::new(Store::new(Counter { n: 0 }));

        std::thread::scope(|scope| {
            let producers: Vec<_> = (0..4)
                .map(|_| {
                    let s = store.clone();
                    scope.spawn(move || {
                        for _ in 0..1000 {
                            s.dispatch(Bump);
                        }
                    })
                })
                .collect();

            let consumer = {
                let s = store.clone();
                scope.spawn(move || {
                    while *s.select(|c| c.n) < 4000 {
                        s.flush();
                        std::thread::yield_now();
                    }
                })
            };

            for p in producers {
                p.join().unwrap();
            }

            consumer.join().unwrap();
        });

        store.flush();

        assert_eq!(store.select(|c| c.n), 4000);
    }

    #[test]
    fn backpressure_blocks_not_drops() {
        use std::sync::Arc;

        let store = Arc::new(Store::new(Counter { n: 0 }).with_capacity(4));

        std::thread::scope(|scope| {
            let producer = {
                let s = store.clone();
                scope.spawn(move || {
                    for _ in 0..1000 {
                        s.dispatch(Bump);
                    }
                })
            };

            // Consumer: periodically flush so the producer can make progress.
            let consumer = {
                let s = store.clone();
                scope.spawn(move || {
                    while *s.select(|c| c.n) < 1000 {
                        s.flush();
                        std::thread::yield_now();
                    }
                })
            };

            producer.join().unwrap();
            consumer.join().unwrap();
        });

        store.flush();

        assert_eq!(store.select(|c| c.n), 1000);
    }

    #[test]
    fn concurrent_flushes_serialize() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let store = Arc::new(Store::new(Counter { n: 0 }));
        let done = Arc::new(AtomicBool::new(false));

        std::thread::scope(|scope| {
            let pusher = {
                let s = store.clone();
                let done = done.clone();

                scope.spawn(move || {
                    for _ in 0..1000 {
                        s.dispatch(Bump);
                    }
                    done.store(true, Ordering::Release);
                })
            };

            let flushers: Vec<_> = (0..2)
                .map(|_| {
                    let s = store.clone();
                    let done = done.clone();

                    scope.spawn(move || {
                        while !done.load(Ordering::Acquire) || *s.select(|c| c.n) < 1000 {
                            s.flush();
                            std::thread::yield_now();
                        }
                    })
                })
                .collect();

            pusher.join().unwrap();

            for f in flushers {
                f.join().unwrap();
            }
        });

        store.flush();

        assert_eq!(store.select(|c| c.n), 1000);
    }
}
