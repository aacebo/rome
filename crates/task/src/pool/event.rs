pub enum Event {
    Task(TaskEvent),
    Thread(ThreadEvent),
}

pub enum TaskEvent {
    Queued,
    Completed,
    Spawned,
}

pub enum ThreadEvent {
    Stopped,
    Spawned,
}
