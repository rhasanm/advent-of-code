use advent_of_code::solutions::year2024::day01::{self, similarity_score, total_distance};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day01::parse_input(EXAMPLE_INPUT)?;

    let res = total_distance(data);
    assert_eq!(res, 11);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day01::solve_part1()?;
    println!("Solution Part 1: {}", solution);
    assert_eq!(solution, "2344935");
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day01::parse_input(EXAMPLE_INPUT)?;

    let res = similarity_score(data);
    assert_eq!(res, 31);
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day01::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    assert_eq!(solution, "27647262");
    Ok(())
}
