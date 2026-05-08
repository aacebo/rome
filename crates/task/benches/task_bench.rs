use std::{path::Path, time::Duration};

use ayr_task::{Task, config::PoolConfig, pool::TaskPool};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use futures::executor::block_on;
use tokio::runtime::Runtime;

// ── dhat criterion profiler ───────────────────────────────────────────────────

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

struct DhatProfiler {
    profiler: Option<dhat::Profiler>,
    before: Option<dhat::HeapStats>,
}

impl DhatProfiler {
    fn new() -> Self {
        Self {
            profiler: None,
            before: None,
        }
    }
}

impl criterion::profiler::Profiler for DhatProfiler {
    fn start_profiling(&mut self, _id: &str, _dir: &Path) {
        self.profiler = Some(dhat::Profiler::builder().testing().build());
        self.before = Some(dhat::HeapStats::get());
    }

    fn stop_profiling(&mut self, id: &str, _dir: &Path) {
        let after = dhat::HeapStats::get();
        if let Some(before) = self.before.take() {
            let allocs = after.total_blocks - before.total_blocks;
            let bytes = after.total_bytes - before.total_bytes;
            eprintln!("  dhat [{id}]  allocs: {allocs}  bytes: {bytes}");
        }
        self.profiler = None;
    }
}

fn custom_criterion() -> Criterion {
    Criterion::default().with_profiler(DhatProfiler::new())
}

// ── ayr-task helpers ──────────────────────────────────────────────────────────

fn make_pool(name: &str) -> TaskPool {
    let pool = TaskPool::new(PoolConfig::new(name));
    pool.start();
    pool
}

// ── tokio helpers ─────────────────────────────────────────────────────────────

fn make_tokio_rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

// ── ayr-task bench bodies ─────────────────────────────────────────────────────

#[inline(never)]
fn bench_spawn_and_await(pool: &TaskPool) {
    let task = pool.spawn(async { 42u64 });
    black_box(block_on(task).unwrap());
}

#[inline(never)]
fn bench_spawn_throughput(pool: &TaskPool) {
    let tasks: Vec<Task<u64>> = (0..100).map(|_| pool.spawn(async { 1u64 })).collect();
    for t in tasks {
        block_on(t).unwrap();
    }
}

#[inline(never)]
fn bench_cancel_task(pool: &TaskPool) {
    let task = pool.spawn(async { 42u64 });
    let cancel = task.cancellation();
    cancel.cancel();
    let _ = black_box(block_on(task));
}

#[inline(never)]
fn bench_pool_scale_up() {
    let pool =
        TaskPool::new(PoolConfig::new("scale-up-bench").with_scale_up_latency(Duration::ZERO));
    pool.start();
    let tasks: Vec<Task<u64>> = (0..10).map(|_| pool.spawn(async { 1u64 })).collect();
    for t in tasks {
        block_on(t).unwrap();
    }
    pool.stop();
}

#[inline(never)]
fn bench_metrics_snapshot(pool: &TaskPool) {
    black_box(pool.metrics());
}

// ── tokio bench bodies ────────────────────────────────────────────────────────

#[inline(never)]
fn bench_tokio_spawn_and_await(rt: &Runtime) {
    black_box(rt.block_on(async { tokio::task::spawn(async { 42u64 }).await.unwrap() }));
}

#[inline(never)]
fn bench_tokio_spawn_throughput(rt: &Runtime) {
    rt.block_on(async {
        let handles: Vec<_> = (0..100)
            .map(|_| tokio::task::spawn(async { 1u64 }))
            .collect();
        for h in handles {
            h.await.unwrap();
        }
    });
}

#[inline(never)]
fn bench_tokio_cancel_task(rt: &Runtime) {
    rt.block_on(async {
        let handle = tokio::task::spawn(async { 42u64 });
        handle.abort();
        let _ = black_box(handle.await);
    });
}

// ── ayr-task criterion fns ────────────────────────────────────────────────────

fn spawn_and_await(c: &mut Criterion) {
    let pool = make_pool("bench-spawn-and-await");
    c.bench_function("spawn_and_await", |b| {
        b.iter(|| bench_spawn_and_await(&pool));
    });
    pool.stop();
}

fn spawn_throughput(c: &mut Criterion) {
    let pool = make_pool("bench-spawn-throughput");
    c.bench_function("spawn_throughput_100", |b| {
        b.iter(|| bench_spawn_throughput(&pool));
    });
    pool.stop();
}

fn cancel_task(c: &mut Criterion) {
    let pool = make_pool("bench-cancel");
    c.bench_function("cancel_task", |b| {
        b.iter(|| bench_cancel_task(&pool));
    });
    pool.stop();
}

fn pool_scale_up(c: &mut Criterion) {
    c.bench_function("pool_scale_up", |b| {
        b.iter(|| bench_pool_scale_up());
    });
}

fn metrics_snapshot(c: &mut Criterion) {
    let pool = make_pool("bench-metrics");
    c.bench_function("metrics_snapshot", |b| {
        b.iter(|| bench_metrics_snapshot(&pool));
    });
    pool.stop();
}

// ── tokio criterion fns ───────────────────────────────────────────────────────

fn tokio_spawn_and_await(c: &mut Criterion) {
    let rt = make_tokio_rt();
    c.bench_function("tokio/spawn_and_await", |b| {
        b.iter(|| bench_tokio_spawn_and_await(&rt));
    });
}

fn tokio_spawn_throughput(c: &mut Criterion) {
    let rt = make_tokio_rt();
    c.bench_function("tokio/spawn_throughput_100", |b| {
        b.iter(|| bench_tokio_spawn_throughput(&rt));
    });
}

fn tokio_cancel_task(c: &mut Criterion) {
    let rt = make_tokio_rt();
    c.bench_function("tokio/cancel_task", |b| {
        b.iter(|| bench_tokio_cancel_task(&rt));
    });
}

// ── groups ────────────────────────────────────────────────────────────────────

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = spawn_and_await, spawn_throughput, cancel_task, pool_scale_up, metrics_snapshot
}

criterion_group! {
    name = tokio_benches;
    config = custom_criterion();
    targets = tokio_spawn_and_await, tokio_spawn_throughput, tokio_cancel_task
}

criterion_main!(benches, tokio_benches);
