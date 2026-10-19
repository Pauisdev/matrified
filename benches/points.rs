use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use matrified::function::compute_points_no_cache;

fn normal_function(c: &mut Criterion) {
    c.bench_function("Square fn [5000 sample size]", |b| {
        b.iter(|| {
            compute_points_no_cache(
                black_box(String::from("x^2")),
                black_box(200),
                black_box(5000),
                black_box(100),
                black_box(10),
            )
        });
    });
}

fn hard_function(c: &mut Criterion) {
    c.bench_function("Hard fn [5000 sample size]", |b| {
        b.iter(|| {
            compute_points_no_cache(
                black_box(String::from("tan(e^x)/x")),
                black_box(200),
                black_box(5000),
                black_box(100),
                black_box(10),
            )
        });
    });
}

fn hard_function_extreme_variant(c: &mut Criterion) {
    c.bench_function("Hard fn extreme variant [500.000 sample size]", |b| {
        b.iter(|| {
            compute_points_no_cache(
                black_box(String::from("tan(e^x)/x")),
                black_box(200),
                black_box(500000),
                black_box(100),
                black_box(10),
            )
        });
    });
}

criterion_group!(
    compute,
    normal_function,
    hard_function,
    hard_function_extreme_variant
);
criterion_main!(compute);
