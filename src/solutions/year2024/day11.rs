use crate::utils::prelude::read_input;
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<i128>> {
    Ok(input
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<i128>().unwrap())
        .collect::<Vec<i128>>())
}

pub fn blink(stones: Vec<i128>) -> Vec<i128> {
    stones
        .iter()
        .flat_map(|&stone| {
            let digit_count = if stone == 0 {
                1
            } else {
                (stone.abs() as f64).log10().floor() as i128 + 1
            };

            if stone == 0 {
                vec![1]
            } else if digit_count % 2 == 0 {
                let power = 10_i128.pow((digit_count / 2) as u32);
                vec![stone / power, stone % power]
            } else {
                vec![stone * 2024]
            }
        })
        .collect()
}

pub fn solve_part1() -> Result<i128> {
    let input = read_input(2024, 11)?;
    let mut stones = parse_input(&input)?;

    for _ in 0..25 {
        stones = blink(stones);
    }

    Ok(stones.len() as i128)
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 11)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
