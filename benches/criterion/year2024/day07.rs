use advent_of_code::{solutions::year2024::day07, utils::prelude::read_input};
use criterion::{black_box, criterion_group, BenchmarkId, Criterion, PlotConfiguration, SamplingMode};

pub fn benchmark_part1(c: &mut Criterion) {
    let input = read_input(2024, 7).unwrap();
    
    let plot_config = PlotConfiguration::default()
        .summary_scale(criterion::AxisScale::Linear);

    let mut group = c.benchmark_group("2024_day07");
    group
        .plot_config(plot_config)
        .sampling_mode(SamplingMode::Linear)
        .sample_size(10);

    group.bench_function("part1/main_solution", |b| {
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

    group.bench_function("part1/parse_input", |b| {
        b.iter(|| {
            black_box(day07::parse_input::<i64>(&input).unwrap())
        })
    });

    let input_sizes = [10, 50, 100, 200];
    for size in input_sizes.iter() {
        let test_input = format!("42: 1 2 3\n").repeat(*size);
        
        group.bench_with_input(
            BenchmarkId::new("part1/scaling", size), 
            &test_input,
            |b, test_input| {
                b.iter(|| {
                    let data = day07::parse_input::<i64>(test_input).unwrap();
                    black_box(
                        data.iter()
                            .filter(|&equation| day07::find_combination_using_binary(equation).unwrap())
                            .map(|equation| equation.test_value)
                            .sum::<i64>()
                    )
                })
            }
        );
    }

    group.finish();
}

pub fn benchmark_part2(c: &mut Criterion) {
    let input = read_input(2024, 7).unwrap();
    let plot_config = PlotConfiguration::default()
    .summary_scale(criterion::AxisScale::Linear);

    let mut group = c.benchmark_group("2024_day07");
    group
    .plot_config(plot_config)
    .sampling_mode(SamplingMode::Linear)
    .sample_size(10);

    group.bench_function("part2/main_solution", |b| {
        b.iter(|| {
            let data = day07::parse_input::<i128>(&input).unwrap();
            black_box(
                data.iter()
                    .filter(|&equation| day07::find_combination_with_concatenating(equation, vec![]).unwrap())
                    .map(|equation| equation.test_value)
                    .sum::<i128>()
            )
        })
    });

    let vector_sizes = [0, 10, 50, 100];
    for size in vector_sizes.iter() {
        group.bench_with_input(
            BenchmarkId::new("part2/vector_capacity", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let data = day07::parse_input::<i128>(&input).unwrap();
                    black_box(
                        data.iter()
                            .filter(|&equation| {
                                let vec = Vec::with_capacity(size);
                                day07::find_combination_with_concatenating(equation, vec).unwrap()
                            })
                            .map(|equation| equation.test_value)
                            .sum::<i128>()
                    )
                })
            }
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_part1, benchmark_part2);