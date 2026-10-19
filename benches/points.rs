use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use matrified::function::compute_points_no_cache;

fn compute_benchmark(c: &mut Criterion) {
    c.bench_function("Compute points", |b| {
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

criterion_group!(benches, compute_benchmark);
criterion_main!(benches);
