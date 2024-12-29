use advent_of_code::solutions::year2024::day07::{self, find_combination, Input};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn test_find_combination() -> Result<()> {
    let data = day07::parse_input(EXAMPLE_INPUT)?;

    let res = find_combination(&data[0], Vec::new()).unwrap();
    assert!(res);
    Ok(())
}

#[test]
fn test_part1_example() -> Result<()> {
    let data = day07::parse_input(EXAMPLE_INPUT)?;

    let total_calibration: i64 = data
        .iter()
        .filter(|&equation| find_combination(equation, vec![]).unwrap())
        .map(|equation| equation.test_value)
        .sum();

    assert_eq!(total_calibration, 3749);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day07::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 7710205485870);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day07::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day07::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
