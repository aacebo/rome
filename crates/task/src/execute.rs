use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{PoolConfig, Task, TaskPool};

pub struct Executor {
    pools: Mutex<HashMap<String, Arc<TaskPool>>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            pools: Mutex::new(HashMap::new()),
        }
    }

    pub fn pool(&self, config: PoolConfig) -> Arc<TaskPool> {
        let mut pools = self.pools.lock().unwrap();

        pools
            .entry(config.name.clone())
            .or_insert_with(|| {
                let p = TaskPool::new(config);
                p.start();
                Arc::new(p)
            })
            .clone()
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

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
