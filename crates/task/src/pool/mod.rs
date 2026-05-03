mod command;
mod config;

pub use command::*;
pub use config::*;

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::Wake,
};

use crate::{
    Task, internal,
    metrics::{PoolMetrics, PoolMetricsSnapshot},
};

pub struct TaskPool {
    next_id: AtomicU64,
    config: PoolConfig,
    stopped: AtomicBool,
    metrics: Arc<PoolMetrics>,
    workers: Mutex<Vec<Arc<internal::Worker>>>,
    commands: internal::Channel<Command>,
}

impl TaskPool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            next_id: AtomicU64::new(0),
            config,
            stopped: AtomicBool::new(false),
            metrics: Arc::new(PoolMetrics::default()),
            workers: Mutex::new(vec![]),
            commands: internal::Channel::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn metrics(&self) -> PoolMetricsSnapshot {
        self.metrics.snapshot()
    }

    pub fn start(&self) {
        let mut workers = self.workers.lock().unwrap();

        if !workers.is_empty() {
            return;
        }

        let worker = Arc::new(internal::Worker::new());

        worker.start(
            self.name(),
            self.metrics.clone(),
            self.commands.receiver().clone(),
        );

        workers.push(worker);
    }

    pub fn stop(&self) {
        if self.stopped.swap(true, Ordering::AcqRel) {
            return;
        }

        let mut workers = self.workers.lock().unwrap();

        for _ in 0..workers.len() {
            let _ = self.commands.sender().send(Command::stop());
        }

        for worker in workers.drain(..) {
            worker.stop();
        }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        if std::time::Duration::from_nanos(self.metrics.tasks.spawn_latency_ns.get())
            >= self.config.scale_up_latency
        {
            self.spawn_thread();
        }

        let run = Arc::new(internal::TaskRun::new(
            self.next_id.fetch_add(1, Ordering::SeqCst).into(),
            self.commands.sender().clone(),
            future,
        ));

        run.wake_by_ref();
        self.metrics.tasks.queued.increment();
        Task { run }
    }

    fn spawn_thread(&self) {
        let worker = Arc::new(internal::Worker::new());

        worker.start(
            self.name(),
            self.metrics.clone(),
            self.commands.receiver().clone(),
        );

        self.workers.lock().unwrap().push(worker);
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn make_pool(name: &str) -> TaskPool {
        TaskPool::new(PoolConfig::new(name))
    }

    async fn with_timeout<F: Future>(f: F) -> F::Output {
        tokio::time::timeout(Duration::from_secs(2), f)
            .await
            .expect("operation hung")
    }

    async fn wait_for_threads(pool: &TaskPool, expected: u64) {
        with_timeout(async {
            while pool.metrics().threads.spawned < expected {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await;
    }

    #[tokio::test]
    async fn name_returns_config_name() {
        let pool = make_pool("custom-name");
        assert_eq!(pool.name(), "custom-name");
    }

    #[tokio::test]
    async fn new_pool_has_no_workers_until_start() {
        let pool = make_pool("lifecycle-new");
        assert_eq!(pool.metrics().threads.spawned, 0);

        pool.start();
        wait_for_threads(&pool, 1).await;
        assert_eq!(pool.metrics().threads.spawned, 1);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn start_is_idempotent() {
        let pool = make_pool("lifecycle-start");
        pool.start();
        pool.start();
        wait_for_threads(&pool, 1).await;
        assert_eq!(pool.metrics().threads.spawned, 1);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn stop_is_idempotent() {
        let pool = make_pool("lifecycle-stop");
        pool.start();

        with_timeout(async {
            pool.stop();
            pool.stop();
        })
        .await;
    }

    #[tokio::test]
    async fn stop_without_start_is_a_noop() {
        let pool = make_pool("lifecycle-stop-no-start");
        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn drop_stops_the_pool() {
        with_timeout(async {
            let pool = make_pool("lifecycle-drop");
            pool.start();
            drop(pool);
        })
        .await;
    }

    #[tokio::test]
    async fn spawn_runs_future_and_returns_value() {
        let pool = make_pool("spawn-value");
        pool.start();

        let task = pool.spawn(async { 42_u32 });
        let out = with_timeout(task).await.expect("task failed");
        assert_eq!(out, 42);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn spawn_before_start_still_runs() {
        let pool = make_pool("spawn-before-start");

        let task = pool.spawn(async { 7_u32 });
        pool.start();

        let out = with_timeout(task).await.expect("task failed");
        assert_eq!(out, 7);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn spawn_increments_queued_metric() {
        let pool = make_pool("metrics-queued");
        pool.start();

        let mut tasks = Vec::new();
        for i in 0..5_u32 {
            tasks.push(pool.spawn(async move { i }));
        }

        for (i, task) in tasks.into_iter().enumerate() {
            let v = with_timeout(task).await.expect("task failed");
            assert_eq!(v, i as u32);
        }

        let snap = pool.metrics();
        assert_eq!(snap.tasks.queued, 5);
        assert_eq!(snap.tasks.completed, 5);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn queued_increments_at_spawn_time() {
        let pool = make_pool("metrics-queued-eager");

        let _t1 = pool.spawn(async { 1_u32 });
        let _t2 = pool.spawn(async { 2_u32 });
        let _t3 = pool.spawn(async { 3_u32 });

        assert_eq!(pool.metrics().tasks.queued, 3);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn many_concurrent_spawns_all_complete() {
        use futures::future::join_all;

        let pool = make_pool("spawn-many");
        pool.start();

        let tasks: Vec<_> = (0..100_u32).map(|i| pool.spawn(async move { i })).collect();
        let results = with_timeout(join_all(tasks)).await;

        for (i, res) in results.into_iter().enumerate() {
            assert_eq!(res.expect("task failed"), i as u32);
        }

        assert_eq!(pool.metrics().tasks.completed, 100);

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn high_latency_spawn_triggers_scale_up() {
        let pool = TaskPool::new(
            PoolConfig::new("scale-up").with_scale_up_latency(Duration::from_nanos(1)),
        );
        pool.start();

        let t1 = pool.spawn(async { 1_u32 });
        with_timeout(t1).await.expect("task failed");

        let before = pool.metrics().threads.spawned;
        let t2 = pool.spawn(async { 2_u32 });
        with_timeout(t2).await.expect("task failed");

        assert!(
            pool.metrics().threads.spawned > before,
            "expected scale-up: before={}, after={}",
            before,
            pool.metrics().threads.spawned
        );

        with_timeout(async { pool.stop() }).await;
    }

    #[tokio::test]
    async fn low_latency_does_not_scale_up() {
        let pool = make_pool("no-scale-up");
        pool.start();

        let task = pool.spawn(async { 1_u32 });
        with_timeout(task).await.expect("task failed");

        assert_eq!(pool.metrics().threads.spawned, 1);

        with_timeout(async { pool.stop() }).await;
    }
}
