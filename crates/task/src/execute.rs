use std::{collections::HashMap, sync::Mutex};

use crate::{Task, TaskPool};

pub struct Executor {
    pools: Mutex<HashMap<String, TaskPool>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            pools: Mutex::new(HashMap::new()),
        }
    }

    pub fn pool(&self, name: impl Into<String>) {
        let name = name.into();
        let max = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let mut pools = self.pools.lock().unwrap();
        let capacity = max / pools.len().max(1);

        pools.insert(name.clone(), TaskPool::new(name, capacity));
    }

    pub fn start(&self) {
        let pools = self.pools.lock().unwrap();

        for (_, pool) in pools.iter() {
            pool.start();
        }
    }

    pub fn stop(&self) {
        let pools = self.pools.lock().unwrap();

        for (_, pool) in pools.iter() {
            pool.stop();
        }
    }

    pub fn spawn<T>(&self, pool: &str, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let pools = self.pools.lock().unwrap();

        match pools.get(pool) {
            None => panic!("pool \"{}\" not found", pool),
            Some(pool) => pool.spawn(future),
        }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        self.stop();
    }
}
