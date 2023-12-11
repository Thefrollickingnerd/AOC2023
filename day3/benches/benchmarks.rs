use day3::part1;
use std::fs;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn part1_benchmark(c: &mut Criterion) {
    let contents = black_box(fs::read_to_string("day3.txt")
        .expect("Should have been able to read the file"));

    c.bench_function("part1", |b| b.iter(move|| part1(contents)));
}

criterion_group!(benches, part1_benchmark);
criterion_main!(benches); 