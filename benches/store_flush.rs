#![feature(test)]
extern crate test;

use rome::state::{Action, Store};
use test::{Bencher, black_box};

#[derive(Clone, Default)]
struct HeavyState {
    items: Vec<u64>,
}

#[derive(Debug)]
struct Push(u64);

impl Action for Push {
    type State = HeavyState;

    fn name(&self) -> &'static str {
        "push"
    }

    fn reduce(&self, state: &mut HeavyState) {
        state.items.push(self.0);
    }
}

#[bench]
fn flush_no_subscribers(b: &mut Bencher) {
    let store = Store::new(HeavyState {
        items: vec![0; 10_000],
    });
    b.iter(|| {
        for i in 0..32 {
            store.dispatch(Push(black_box(i)));
        }
        store.flush();
        black_box(&store);
    });
}

#[bench]
fn flush_one_subscriber(b: &mut Bencher) {
    let store = Store::new(HeavyState {
        items: vec![0; 10_000],
    });
    let s = store.select(|s| s.items.len());
    b.iter(|| {
        for i in 0..32 {
            store.dispatch(Push(black_box(i)));
        }
        store.flush();
        black_box(s.get());
    });
}
