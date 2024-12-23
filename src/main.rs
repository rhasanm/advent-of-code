use advent_of_code::solutions::year2024;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: cargo run <year> <day> <part>");
        println!("Example: cargo run 2023 1 1");
        return Ok(());
    }

    let year = args[1].parse::<u32>()?;
    let day = args[2].parse::<u32>()?;
    let part = args[3].parse::<u32>()?;

    match (year, day, part) {
        (2024, 1, 1) => println!("Solution: {}", year2024::day01::solve_part1()?),
        (2024, 1, 2) => println!("Solution: {}", year2024::day01::solve_part2()?),
        _ => println!(
            "Solution not implemented yet for year {} day {} part {}",
            year, day, part
        ),
    }

    Ok(())
}
