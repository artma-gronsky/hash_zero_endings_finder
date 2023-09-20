use criterion::{criterion_group, criterion_main, Criterion};
use std::process::Command;

fn find_zero_symbols_without_specifying_concurrent_param() {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("hash_zero_endings_finder")
        .arg("--")
        .arg("-N")
        .arg("3")
        .arg("-F")
        .arg("4")
        .output();
}

fn find_zero_symbols_with_specifying_concurrent_param(c: usize) {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("hash_zero_endings_finder")
        .arg("--")
        .arg("-N")
        .arg("3")
        .arg("-F")
        .arg("4")
        .arg("--concurrent")
        .arg(c.to_string())
        .output();
}

fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("finder", |b| {
        b.iter(|| find_zero_symbols_without_specifying_concurrent_param())
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("finder_concurrent_1", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(1))
    });
}
fn criterion_benchmark3(c: &mut Criterion) {
    c.bench_function("finder_concurrent_2", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(2))
    });
}

fn criterion_benchmark4(c: &mut Criterion) {
    c.bench_function("finder_concurrent_3", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(3))
    });
}

fn criterion_benchmark5(c: &mut Criterion) {
    c.bench_function("finder_concurrent_4", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(4))
    });
}

fn criterion_benchmark6(c: &mut Criterion) {
    c.bench_function("finder_concurrent_5", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(5))
    });
}

fn criterion_benchmark7(c: &mut Criterion) {
    c.bench_function("finder_concurrent_6", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(6))
    });
}

fn criterion_benchmark8(c: &mut Criterion) {
    c.bench_function("finder_concurrent_12", |b| {
        b.iter(|| find_zero_symbols_with_specifying_concurrent_param(12))
    });
}

criterion_group!(
    benches,
    criterion_benchmark1,
    criterion_benchmark2,
    criterion_benchmark3,
    criterion_benchmark4,
    criterion_benchmark5,
    criterion_benchmark6,
    criterion_benchmark7,
    criterion_benchmark8
);
criterion_main!(benches);
