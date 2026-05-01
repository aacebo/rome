use crate::TaskId;

pub enum Event {
    Task(TaskEvent),
    Thread(ThreadEvent),
}

pub enum TaskEvent {
    Queued(TaskId),
    Completed(TaskId),
    Spawned(TaskId),
}

impl From<TaskEvent> for Event {
    fn from(value: TaskEvent) -> Self {
        Self::Task(value)
    }
}

pub enum ThreadEvent {
    Stopped(std::thread::ThreadId),
    Spawned(std::thread::ThreadId),
}

impl From<ThreadEvent> for Event {
    fn from(value: ThreadEvent) -> Self {
        Self::Thread(value)
    }
}
