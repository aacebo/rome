// Apples-to-apples comparison between rome's Signal primitive and
// tokio::sync::watch. We compare the lowest-level mutate+notify path
// (Signal::with_mut vs watch::Sender::send_modify) and the replace+notify
// path (Signal::emit vs watch::Sender::send), each with one passive subscriber
// (subscriber exists but is not actively polling/awaiting).
//
// State shape is identical across both: Vec<u64> of 10,000 entries.
// Per iteration: 32 mutations, then dropped/replaced, exactly the same shape
// as the workload in store_flush.rs minus the dispatch/reduce queue layer.

#![feature(test)]
extern crate test;

use rome::state::Signal;
use test::{Bencher, black_box};
use tokio::sync::watch;

#[derive(Clone, Default)]
struct HeavyState {
    items: Vec<u64>,
}

fn make_heavy() -> HeavyState {
    HeavyState {
        items: vec![0; 10_000],
    }
}

// ---- in-place mutation + notify ----

#[bench]
fn rome_with_mut_32x(b: &mut Bencher) {
    let signal = Signal::new(make_heavy());
    // Hold a Reader so there's an outstanding subscriber, but don't poll it.
    let _reader = signal.stream();
    b.iter(|| {
        signal.with_mut(|s| {
            for i in 0..32 {
                s.items.push(black_box(i));
            }
        });
        black_box(&signal);
    });
}

#[bench]
fn tokio_send_modify_32x(b: &mut Bencher) {
    let (tx, _rx) = watch::channel(make_heavy());
    b.iter(|| {
        tx.send_modify(|s| {
            for i in 0..32 {
                s.items.push(black_box(i));
            }
        });
        black_box(&tx);
    });
}

// ---- replace + notify ----
//
// Both rome's Signal::emit and watch::Sender::send replace the inner value.
// Per iteration we build a fresh HeavyState (10,000-element vec) and push it
// through. This is the worst case for our path because we lose the
// Arc::make_mut shortcut and pay a full allocation per iter.

#[bench]
fn rome_emit_32x(b: &mut Bencher) {
    let signal = Signal::new(make_heavy());
    let _reader = signal.stream();
    b.iter(|| {
        for i in 0..32 {
            let mut next = (*signal.get()).clone();
            next.items.push(black_box(i));
            signal.emit(next);
        }
        black_box(&signal);
    });
}

#[bench]
fn tokio_send_replace_32x(b: &mut Bencher) {
    let (tx, _rx) = watch::channel(make_heavy());
    b.iter(|| {
        for i in 0..32 {
            let mut next = tx.borrow().clone();
            next.items.push(black_box(i));
            let _ = tx.send(next);
        }
        black_box(&tx);
    });
}
