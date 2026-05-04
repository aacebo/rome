use std::time::Duration;

#[derive(Clone)]
pub struct PoolConfig {
    pub name: String,
    pub max: usize,
    pub scale_up_latency: Duration,
}

impl PoolConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            max: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            scale_up_latency: Duration::from_millis(5),
        }
    }

    pub fn with_max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    pub fn with_scale_up_latency(mut self, latency: Duration) -> Self {
        self.scale_up_latency = latency;
        self
    }
}
