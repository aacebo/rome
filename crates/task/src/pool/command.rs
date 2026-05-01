use std::sync::Arc;

use crate::{Event, Job};

pub enum Command {
    Stop,
    Run(Arc<dyn Job>),
    Emit(Event),
}
