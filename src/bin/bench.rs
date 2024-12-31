use advent_of_code::benchmark::{year2024::Day07Benchmark, BenchmarkRunner};
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "bench")]
struct Cli {
    #[arg(short = 'y', long = "year", value_name = "YEAR")]
    year: u16,
    
    #[arg(short = 'd', long = "day", value_name = "DAY")]
    day: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runner = BenchmarkRunner::new();
    
    match (cli.year, cli.day) {
        (2024, 7) => runner.run::<Day07Benchmark>()?,
        _ => anyhow::bail!("Year {} Day {} not implemented", cli.year, cli.day),
    }
    
    Ok(())
}