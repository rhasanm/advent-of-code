use super::{Benchmark, BenchmarkRunner};
use crate::solutions::year2024::day07::{self, Input};
use anyhow::Result;
use criterion::Criterion;
use std::fs;

pub struct Day07Benchmark {
    data: Vec<Input<i64>>,
}

impl Benchmark for Day07Benchmark {
    fn new() -> Result<Self> {
        let runner = BenchmarkRunner::new();
        let input = fs::read_to_string(runner.get_input_path(2024, 7))?;
        let data = day07::parse_input(&input)?;
        Ok(Self { data })
    }

    fn run_benchmark(&self, c: &mut Criterion) {
        let mut group = c.benchmark_group("Day07");
        
        group.bench_function("binary_approach", |b| {
            b.iter(|| {
                for equation in criterion::black_box(&self.data) {
                    let _ = day07::find_combination_using_binary(equation);
                }
            })
        });

        group.bench_function("recursive_approach", |b| {
            b.iter(|| {
                for equation in criterion::black_box(&self.data) {
                    let _ = day07::find_combination(equation, Vec::new());
                }
            })
        });

        group.finish();
    }
}