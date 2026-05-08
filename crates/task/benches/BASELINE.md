# `ayr-task` performance baseline

Side-by-side comparison of `ayr-task` against tokio's multi-thread runtime.
Each bench runs the same logical operation through both executors so the
overhead of our custom scheduler is visible at a glance.

## How to reproduce

```powershell
cargo bench -p ayr-task
```

Criterion writes HTML reports to `target/criterion/`.

## Environment

- Host: Windows 11, win32 toolchain, `cargo` default release profile.
- Date captured: 2026-05-08.
- Rust: workspace pin (Edition 2024).

## Baseline

| Bench | `ayr-task` | `tokio` |
|---|---|---|
| `spawn_and_await` | 1.09 µs | 30.8 µs |
| `spawn_throughput_100` | 35.4 µs | 45.6 µs |
| `cancel_task` | 1.15 µs | 32.8 µs |
| `pool_scale_up` | 885 µs | — |
| `metrics_snapshot` | 2.44 ns | — |

### Reading the numbers

- **`spawn_and_await`** — full round-trip: one no-op future queued, executed by a
  worker thread, and joined. Measures raw scheduler + wake overhead per task.
- **`spawn_throughput_100`** — 100 tasks spawned and drained sequentially.
  Reveals amortised queue/dispatch cost and how well the pool saturates a single
  worker under light load.
- **`cancel_task`** — spawn a task, immediately cancel it, then join. Measures
  the cancellation fast-path cost. tokio uses `JoinHandle::abort()`; ayr-task
  uses `Cancellation::cancel()`.
- **`pool_scale_up`** — configures `scale_up_latency = Duration::ZERO` so every
  spawn triggers a new worker thread. Measures the cost of dynamic thread
  allocation. No tokio equivalent (tokio's thread count is fixed at runtime
  creation); this bench is ayr-task only.
- **`metrics_snapshot`** — atomic snapshot of pool metrics under no load.
  tokio has no public per-pool metrics API, so this bench is ayr-task only.
