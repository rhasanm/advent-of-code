use advent_of_code::benchmark::{BenchmarkRunner, Day07Benchmark};
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "bench")]
struct Cli {
    #[arg(short = 'd', long = "day", value_name = "DAY")]
    day: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runner = BenchmarkRunner::new();
    
    match cli.day {
        7 => runner.run::<Day07Benchmark>()?,
        _ => anyhow::bail!("Day {} not implemented", cli.day),
    }

    Ok(())
}