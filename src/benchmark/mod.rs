use criterion::Criterion;
use anyhow::Result;
use std::path::PathBuf;

pub mod year2024;

pub trait Benchmark: Sized {
    fn new() -> Result<Self>;
    fn run_benchmark(&self, c: &mut Criterion);
}

pub struct BenchmarkRunner {
    input_dir: PathBuf,
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        Self {
            input_dir: PathBuf::from("inputs"),
        }
    }

    pub fn run<B: Benchmark>(&self) -> Result<()> {
        let benchmark = B::new()?;
        let mut criterion = Criterion::default();
        benchmark.run_benchmark(&mut criterion);
        Ok(())
    }

    pub fn get_input_path(&self, year: u16, day: u8) -> PathBuf {
        self.input_dir
            .join(year.to_string())
            .join(format!("day{:02}.txt", day))
    }
}