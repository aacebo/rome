use std::sync::Arc;

use crate::Job;

pub enum Command {
    Stop,
    Run(Arc<dyn Job>),
}
