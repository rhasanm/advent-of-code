use advent_of_code::{solutions::year2024::day07, utils::prelude::read_input};
use criterion::{black_box, criterion_group, Criterion};

pub fn benchmark_part1(c: &mut Criterion) {
    let input = read_input(2024, 7).unwrap();
    
    let mut group = c.benchmark_group("2024_day07");
    group.bench_function("part1", |b| {
        b.iter(|| {
            let data = day07::parse_input::<i64>(&input).unwrap();
            black_box(
                data.iter()
                    .filter(|&equation| day07::find_combination_using_binary(equation).unwrap())
                    .map(|equation| equation.test_value)
                    .sum::<i64>()
            )
        })
    });
    group.finish();
}

pub fn benchmark_part2(c: &mut Criterion) {
    let input = read_input(2024, 7).unwrap();
    
    let mut group = c.benchmark_group("2024_day07");
    group.bench_function("part2", |b| {
        b.iter(|| {
            let data = day07::parse_input::<i128>(&input).unwrap();
            black_box(
                data.iter()
                    .filter(|&equation| day07::find_combination(equation, vec![]).unwrap())
                    .map(|equation| equation.test_value)
                    .sum::<i128>()
            )
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark_part1, benchmark_part2);