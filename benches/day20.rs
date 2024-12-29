use advent_of_code::presents_delivered_to_house_part1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day20 presents delivered to house", |b| {
        b.iter(|| presents_delivered_to_house_part1(black_box(2000000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
