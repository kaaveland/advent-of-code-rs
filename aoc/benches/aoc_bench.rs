use aoc::YEARS;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn setup_bench_functions(c: &mut Criterion) {
    for (year, solutions) in YEARS {
        let mut group = c.benchmark_group(year.to_string());
        for solution in solutions {
            let day = format!("{:0>2}", solution.day_no);
            let path = format!("../input/{year}/day_{day}/input");
            if let Ok(input) = fs::read_to_string(path) {
                group.bench_function(format!("{day}/part 1"), |b| {
                    b.iter(|| (solution.part_1)(black_box(input.as_str())).unwrap())
                });
                group.bench_function(format!("{day}/part 2"), |b| {
                    b.iter(|| (solution.part_2)(black_box(input.as_str())).unwrap())
                });
            }
        }
    }
}
criterion_group!(benches, setup_bench_functions);
criterion_main!(benches);
