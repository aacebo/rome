mod action;
pub mod signal;
mod store;

pub use action::*;
pub use signal::Signal;
pub use store::*;

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

    fn poll_once<S: futures::Stream + Unpin>(s: &mut S) -> std::task::Poll<Option<S::Item>> {
        let waker = futures::task::noop_waker();
        let mut cx = std::task::Context::from_waker(&waker);
        std::pin::Pin::new(s).poll_next(&mut cx)
    }

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
            assert_eq!(store.select(|s| s.name.clone()), "test user");

            store.flush();
            assert_eq!(store.select(|s| s.name.clone()), "hello world");
        }

        #[test]
        fn flush_applies_dispatched_actions_in_order() {
            let store = Store::new(UserState::default());

            store.dispatch(UserAction::Rename("a".to_string()));
            store.dispatch(UserAction::Rename("b".to_string()));
            store.dispatch(UserAction::Rename("c".to_string()));
            store.flush();

            assert_eq!(store.select(|s| s.name.clone()), "c");
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
                        while s.select(|c| c.n).get() < 4000 {
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
                        while s.select(|c| c.n).get() < 1000 {
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
                            while !done.load(Ordering::Acquire) || s.select(|c| c.n).get() < 1000 {
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

    mod select_stream {
        use super::*;
        use std::task::Poll;

        #[test]
        fn observes_dispatched_changes() {
            let store = Store::new(Counter { n: 0 });
            let n = store.select(|c| c.n);
            let mut sub = Box::pin(n.stream());

            // Drain the seeded current value.
            match poll_once(&mut sub) {
                Poll::Ready(Some(v)) => assert_eq!(v, 0),
                other => panic!("expected Ready(Some(0)), got {:?}", other.map(|_| ())),
            }

            store.dispatch(Bump);
            store.flush();

            match poll_once(&mut sub) {
                Poll::Ready(Some(v)) => assert_eq!(v, 1),
                other => panic!("expected Ready(Some(1)), got {:?}", other.map(|_| ())),
            }
        }

        #[test]
        fn coalesces_across_flushes() {
            let store = Store::new(Counter { n: 0 });
            let n = store.select(|c| c.n);
            let mut sub = Box::pin(n.stream());

            let _ = poll_once(&mut sub); // drain seed

            // Three separate flushes between polls — only the latest is visible.
            store.dispatch(Bump);
            store.flush();
            store.dispatch(Bump);
            store.flush();
            store.dispatch(Bump);
            store.flush();

            match poll_once(&mut sub) {
                Poll::Ready(Some(v)) => assert_eq!(v, 3),
                other => panic!("expected Ready(Some(3)), got {:?}", other.map(|_| ())),
            }
            assert!(matches!(poll_once(&mut sub), Poll::Pending));
        }

        #[test]
        fn multiple_subscribers_each_receive() {
            let store = Store::new(Counter { n: 0 });
            let n = store.select(|c| c.n);
            let mut s1 = Box::pin(n.stream());
            let mut s2 = Box::pin(n.stream());

            let _ = poll_once(&mut s1);
            let _ = poll_once(&mut s2);

            store.dispatch(Bump);
            store.flush();

            for s in [&mut s1, &mut s2] {
                match poll_once(s) {
                    Poll::Ready(Some(v)) => assert_eq!(v, 1),
                    other => panic!("expected Ready(Some(1)), got {:?}", other.map(|_| ())),
                }
            }
        }
    }
}
