//! Performance baseline for ayr-reflect.
//!
//! Two run modes:
//!   * `cargo bench -p ayr-reflect`                      — criterion timing.
//!   * `cargo bench -p ayr-reflect --features dhat-heap` — runs each bench
//!     once under dhat to capture allocation counts. Output is written to
//!     `dhat-heap.json` in the workspace root; counts are also printed.

#![allow(unused)]

use ayr_reflect::{AsValue, ToType, ToValue, TypeOf};
use ayr_reflect_macros::Reflect;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// ----- shared sample types -----

#[derive(Debug, Clone, Reflect)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub score: f64,
}

fn sample_user() -> User {
    User {
        id: 42,
        name: "alex".to_string(),
        email: "alex@example.com".to_string(),
        active: true,
        score: 99.5,
    }
}

fn sample_strings() -> Vec<String> {
    vec!["a".to_string(), "b".to_string(), "c".to_string()]
}

// ----- bench bodies (also reused under dhat) -----

#[inline(never)]
fn bench_type_of_struct() -> ayr_reflect::Type {
    <User as TypeOf>::type_of()
}

#[inline(never)]
fn bench_assignable_to_primitive() -> bool {
    let lhs = i32::type_of();
    let rhs = i32::type_of();
    lhs.assignable_to(rhs)
}

#[inline(never)]
fn bench_clone_struct_type(t: &ayr_reflect::Type) -> ayr_reflect::Type {
    t.clone()
}

#[inline(never)]
fn bench_to_value_vec_string(v: Vec<String>) -> ayr_reflect::Value {
    v.to_value()
}

#[inline(never)]
fn bench_serialize_object_json(user: &User) -> String {
    let dynamic = ayr_reflect::Dynamic::from_object(user.clone());
    serde_json::to_string(&dynamic).expect("serialize")
}

// ----- criterion main (default) -----

#[cfg(not(feature = "dhat-heap"))]
use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(not(feature = "dhat-heap"))]
fn type_of_struct(c: &mut Criterion) {
    c.bench_function("type_of_struct", |b| {
        b.iter(|| black_box(bench_type_of_struct()));
    });
}

#[cfg(not(feature = "dhat-heap"))]
fn assignable_to_primitive(c: &mut Criterion) {
    c.bench_function("assignable_to_primitive", |b| {
        b.iter(|| black_box(bench_assignable_to_primitive()));
    });
}

#[cfg(not(feature = "dhat-heap"))]
fn clone_struct_type(c: &mut Criterion) {
    let t = <User as TypeOf>::type_of();
    c.bench_function("clone_struct_type", |b| {
        b.iter(|| black_box(bench_clone_struct_type(&t)));
    });
}

#[cfg(not(feature = "dhat-heap"))]
fn to_value_vec_string(c: &mut Criterion) {
    c.bench_function("to_value_vec_string", |b| {
        b.iter_with_setup(sample_strings, |v| black_box(bench_to_value_vec_string(v)));
    });
}

#[cfg(not(feature = "dhat-heap"))]
fn serialize_object_json(c: &mut Criterion) {
    let user = sample_user();
    c.bench_function("serialize_object_json", |b| {
        b.iter(|| black_box(bench_serialize_object_json(&user)));
    });
}

#[cfg(not(feature = "dhat-heap"))]
criterion_group!(
    benches,
    type_of_struct,
    assignable_to_primitive,
    clone_struct_type,
    to_value_vec_string,
    serialize_object_json,
);

#[cfg(not(feature = "dhat-heap"))]
criterion_main!(benches);

// ----- dhat main (feature gated) -----
//
// Runs each bench body N times under the dhat heap profiler so the
// per-call allocation count is N* the per-iteration cost. We use N=10000
// for benches that allocate per-iteration (so the relative cost dominates
// the dhat fixed overhead).

#[cfg(feature = "dhat-heap")]
fn main() {
    let _profiler = dhat::Profiler::builder().testing().build();

    const ITERS: usize = 10_000;

    // Snapshot before/after each bench for delta accounting.
    let mut report = Vec::<(String, dhat::HeapStats, dhat::HeapStats)>::new();

    {
        let before = dhat::HeapStats::get();
        for _ in 0..ITERS {
            std::hint::black_box(bench_type_of_struct());
        }
        let after = dhat::HeapStats::get();
        report.push(("type_of_struct".into(), before, after));
    }

    {
        let before = dhat::HeapStats::get();
        for _ in 0..ITERS {
            std::hint::black_box(bench_assignable_to_primitive());
        }
        let after = dhat::HeapStats::get();
        report.push(("assignable_to_primitive".into(), before, after));
    }

    {
        let t = <User as TypeOf>::type_of();
        let before = dhat::HeapStats::get();
        for _ in 0..ITERS {
            std::hint::black_box(bench_clone_struct_type(&t));
        }
        let after = dhat::HeapStats::get();
        report.push(("clone_struct_type".into(), before, after));
    }

    {
        let before = dhat::HeapStats::get();
        for _ in 0..ITERS {
            std::hint::black_box(bench_to_value_vec_string(sample_strings()));
        }
        let after = dhat::HeapStats::get();
        report.push(("to_value_vec_string".into(), before, after));
    }

    {
        let user = sample_user();
        let before = dhat::HeapStats::get();
        for _ in 0..ITERS {
            std::hint::black_box(bench_serialize_object_json(&user));
        }
        let after = dhat::HeapStats::get();
        report.push(("serialize_object_json".into(), before, after));
    }

    println!();
    println!("dhat baseline (averaged over {} iterations):", ITERS);
    println!(
        "{:<28} {:>14} {:>14} {:>16}",
        "bench", "allocs/iter", "bytes/iter", "max_live_bytes"
    );
    for (name, before, after) in &report {
        let allocs = (after.total_blocks - before.total_blocks) as f64 / ITERS as f64;
        let bytes = (after.total_bytes - before.total_bytes) as f64 / ITERS as f64;
        let max_live = after.max_bytes;
        println!(
            "{:<28} {:>14.2} {:>14.0} {:>16}",
            name, allocs, bytes, max_live
        );
    }
}
