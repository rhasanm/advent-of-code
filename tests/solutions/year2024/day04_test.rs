use advent_of_code::solutions::year2024::day04;
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
first line
second line
third line";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day04::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day04::solve_part1()?;
    println!("Solution Part 1: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day04::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day04::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}