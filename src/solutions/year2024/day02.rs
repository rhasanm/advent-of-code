use anyhow::{Ok, Result};
use crate::utils;

pub fn parse_input(input: &str) -> Result<Vec<Vec<i128>>> {
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let numbers: Vec<Vec<i128>> = lines
    .iter()
    .filter_map(|line| {
        let nums: Vec<i128> = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<i128>().ok())
            .collect();
        if nums.is_empty() {
            None
        } else {
            Some(nums)
        }
    })
    .collect();

    Ok(numbers)
}

pub fn check_safe_reports(data: Vec<Vec<i128>>) -> String {
    let candidates_after_differ_check: Vec<Vec<i128>> = data
        .iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|pair| {
                    let diff = (pair[0] - pair[1]).abs();
                    1 <= diff && diff <= 3
                })
        })
        .cloned()
        .collect();

    println!("{:?}", candidates_after_differ_check);

    let sds: Vec<Vec<i128>> = candidates_after_differ_check.iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|pair| {
                    pair[0] < pair[1]
                })
        })
        .cloned()
        .collect();

    let sis: Vec<Vec<i128>> = candidates_after_differ_check.iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|pair| {
                    pair[0] > pair[1]
                })
        })
        .cloned()
        .collect();

    (sis.len() + sds.len()).to_string()
}

pub fn solve_part1() -> Result<String> {
    let input = utils::read_input(2024, 02)?;
    let data = parse_input(&input)?;

    let tot = check_safe_reports(data);
    Ok(tot)
}