use crate::utils::prelude::read_input;
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn solve_part1() -> Result<String> {
    let input = read_input(2024, 10)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 10)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
