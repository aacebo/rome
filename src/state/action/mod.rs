mod buffer;

pub use buffer::*;

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
