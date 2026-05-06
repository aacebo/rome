pub mod action;
pub mod select;
pub mod store;
pub mod trigger;
mod version;

pub use action::{Action, Next};
pub use select::Selector;
pub use store::Store;
pub use trigger::Trigger;
pub use version::Version;

pub struct State<T> {
    version: Version,
    inner: T,
}

impl<T> State<T> {
    pub fn new(inner: T) -> Self {
        Self {
            version: Version::default(),
            inner,
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn take(self) -> T {
        self.inner
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn set(&mut self, next: T) {
        self.version.increment();
        self.inner = next;
    }
}

impl<T> From<T> for State<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> AsRef<T> for State<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsMut<T> for State<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> std::ops::Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::fmt::Debug for State<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self.inner)
    }
}

impl<T> serde::Serialize for State<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for State<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(State::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, Debug, PartialEq)]
    struct UserState {
        pub name: String,
    }

    impl Selector<UserState> for String {
        fn select(state: &UserState) -> &Self {
            &state.name
        }
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

    impl Selector<Counter> for u32 {
        fn select(state: &Counter) -> &Self {
            &state.n
        }
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

    #[derive(Debug)]
    struct Double;

    impl Action for Double {
        type State = Counter;

        fn name(&self) -> &'static str {
            "double"
        }

        fn reduce(&self, state: &mut Counter) {
            state.n *= 2;
        }
    }

    mod dispatch {
        use super::*;

        #[test]
        fn dispatch_is_lazy_until_flush() {
            let mut store = Store::new(UserState {
                name: "test user".to_string(),
            });

            store.dispatch(UserAction::Rename("hello world".to_string()));
            assert_eq!(&*store.select::<String>(), "test user");

            store.flush();
            assert_eq!(&*store.select::<String>(), "hello world");
        }

        #[test]
        fn flush_applies_dispatched_actions_in_order() {
            let mut store = Store::new(UserState::default());

            store.dispatch(UserAction::Rename("a".to_string()));
            store.dispatch(UserAction::Rename("b".to_string()));
            store.dispatch(UserAction::Rename("c".to_string()));
            store.flush();

            assert_eq!(&*store.select::<String>(), "c");
        }
    }

    mod selector {
        use super::*;

        #[test]
        fn reprojects_after_flush() {
            // Selector is a snapshot, not a live view: it captures the state
            // at `select()` time and never updates. A second `select()` after
            // a flush sees the new value.
            let mut store = Store::new(Counter { n: 0 });

            assert_eq!(*store.select::<u32>(), 0);

            store.dispatch(Bump);
            store.flush();

            assert_eq!(*store.select::<u32>(), 1);
        }
    }

    mod trigger {
        use super::*;

        #[test]
        fn executes_on_action() {
            let mut store = Store::new(Counter { n: 5 });

            store.trigger(|_state: &Counter, _action: &Bump, next: &Next<Counter>| {
                next.dispatch(Double);
            });

            store.dispatch(Bump);
            store.flush();
            assert_eq!(*store.select::<u32>(), 12);
        }
    }
}
