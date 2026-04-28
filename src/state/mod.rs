mod action;
mod select;
mod store;

pub use action::*;
pub use select::*;
pub use store::*;

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

    mod dispatch {
        use super::*;

        #[test]
        fn dispatch_is_lazy_until_flush() {
            let store = Store::new(UserState {
                name: "test user".to_string(),
            });

            store.dispatch(UserAction::Rename("hello world".to_string()));
            assert_eq!(*store.select(|s| s.name.clone()), "test user");

            store.flush();
            assert_eq!(*store.select(|s| s.name.clone()), "hello world");
        }

        #[test]
        fn flush_applies_dispatched_actions_in_order() {
            let store = Store::new(UserState::default());

            store.dispatch(UserAction::Rename("a".to_string()));
            store.dispatch(UserAction::Rename("b".to_string()));
            store.dispatch(UserAction::Rename("c".to_string()));
            store.flush();

            assert_eq!(*store.select(|s| s.name.clone()), "c");
        }
    }

    mod selector {
        use super::*;

        #[test]
        fn reprojects_after_flush() {
            // Selector is a snapshot, not a live view: it captures the state
            // at `select()` time and never updates. A second `select()` after
            // a flush sees the new value.
            let store = Store::new(Counter { n: 0 });

            assert_eq!(*store.select(|c| c.n), 0);

            store.dispatch(Bump);
            store.flush();

            assert_eq!(*store.select(|c| c.n), 1);
        }
    }

    mod concurrency {
        use super::*;
        use std::sync::Arc;

        #[test]
        fn multi_producer_push() {
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
            assert_eq!(*store.select(|c| c.n), 4000);
        }

        #[test]
        fn backpressure_does_not_drop_pushes() {
            // With a tiny buffer (capacity 4) and 1000 pushes from one
            // producer, `dispatch` must block when full rather than drop —
            // otherwise the final count would be < 1000.
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
            assert_eq!(*store.select(|c| c.n), 1000);
        }

        #[test]
        fn concurrent_flushes_are_safe() {
            // Two flushers race a single pusher. Serialization comes from
            // the `RwLock` write guard inside `flush`; this test asserts the
            // race produces the correct final count and never deadlocks.
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
            assert_eq!(*store.select(|c| c.n), 1000);
        }
    }
}
