use advent_of_code::solutions::year2024::day02::{self, check_safe_reports};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day02::parse_input(EXAMPLE_INPUT)?;

    let res = check_safe_reports(data);
    assert_eq!(res, "2");
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day02::solve_part1()?;
    println!("Solution Part 1: {}", solution);
    assert_eq!(solution, "510");
    Ok(())
}
