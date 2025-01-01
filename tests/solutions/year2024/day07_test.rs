use advent_of_code::solutions::year2024::day07::{
    self, find_combination, find_combination_using_binary, find_combination_with_concatenating,
};
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
    let data = day07::parse_input::<i64>(EXAMPLE_INPUT)?;

    let res = find_combination(&data[0], Vec::new()).unwrap();
    assert!(res);
    Ok(())
}

#[test]
fn test_find_combination_binary() -> Result<()> {
    let data = day07::parse_input::<i64>(EXAMPLE_INPUT)?;

    let res = find_combination_using_binary(&data[8]).unwrap();
    assert!(res);
    Ok(())
}

#[test]
fn test_find_combination_with_concatenating() -> Result<()> {
    let data = day07::parse_input::<i64>(EXAMPLE_INPUT)?;

    let res = find_combination_with_concatenating(&data[3], Vec::new()).unwrap();
    assert!(res);
    Ok(())
}

#[test]
fn test_part1_example() -> Result<()> {
    let data = day07::parse_input::<i64>(EXAMPLE_INPUT)?;

    let total_calibration: i64 = data
        .iter()
        .filter(|&equation| find_combination_using_binary(equation).unwrap())
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
    let data = day07::parse_input::<i128>(EXAMPLE_INPUT)?;

    let total_calibration: i128 = data
        .iter()
        .filter(|&equation| find_combination_with_concatenating(equation, vec![]).unwrap())
        .map(|equation| equation.test_value)
        .sum();

    assert_eq!(total_calibration, 11387);
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day07::solve_part2()?;
    println!("Solution Part 2: {}", solution);

    assert_eq!(solution, 20928985450275);
    Ok(())
}
