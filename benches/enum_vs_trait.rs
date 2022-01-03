use enum_vs_trait_lib::{quxit_enum, quxit_trait, quxit_generic, quxit_genenum};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn enum_benchmark(c: &mut Criterion) {
    c.bench_function("enum", |b| {
        b.iter(|| quxit_enum(black_box(&[3, 10, -2, -3]), black_box(10), black_box(3)))
    });
}

fn trait_benchmark(c: &mut Criterion) {
    c.bench_function("trait", |b| {
        b.iter(|| quxit_trait(black_box(&[3, 10, -2, -3]), black_box(10), black_box(3)))
    });
}

fn generic_benchmark(c: &mut Criterion) {
    c.bench_function("generic trait", |b| {
        b.iter(|| quxit_generic::<i32>(black_box(&[3, 10, -2, -3]), black_box(10), black_box(3)))
    });
}

fn genenum_benchmark(c: &mut Criterion) {
    c.bench_function("generic enum", |b| {
        b.iter(|| quxit_genenum::<i32>(black_box(&[3, 10, -2, -3]), black_box(10), black_box(3)))
    });
}

criterion_group!(benches, enum_benchmark, trait_benchmark, generic_benchmark, genenum_benchmark);
criterion_main!(benches);
