use std::{hint::black_box, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use matrified::function::compute_points_no_cache;

fn normal_function(c: &mut Criterion) {
    c.bench_function("Square fn [5K sample size]", |b| {
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
    c.bench_function("Hard fn [5K sample size]", |b| {
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

fn hard_function_hard_variant(c: &mut Criterion) {
    c.bench_function("Hard fn hard variant [500K sample size]", |b| {
        b.iter(|| {
            compute_points_no_cache(
                black_box(String::from("tan(e^x)/x")),
                black_box(200),
                black_box(500_000),
                black_box(100),
                black_box(10),
            )
        });
    });
}

fn hard_function_extreme_variant(c: &mut Criterion) {
    c.bench_function("Hard fn extreme variant [5M sample size]", |b| {
        b.iter(|| {
            compute_points_no_cache(
                black_box(String::from("tan(e^x)/x")),
                black_box(200),
                black_box(5_000_000),
                black_box(100),
                black_box(10),
            )
        });
    });
}

criterion_group!(
    name = normal_compute;
    config = Criterion::default().measurement_time(Duration::from_secs(7));
    targets =
        normal_function,
        hard_function,
);

criterion_group!(
    name = hard_compute;
    config = Criterion::default().sample_size(50).measurement_time(Duration::from_secs(10));
    targets =
        hard_function_hard_variant,
);

criterion_group!(
    name = extreme_compute;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(15));
    targets =
        hard_function_extreme_variant
);

criterion_main!(normal_compute, hard_compute, extreme_compute);
