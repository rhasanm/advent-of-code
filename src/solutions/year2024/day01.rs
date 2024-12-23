use std::{collections::HashMap, iter::zip};

use crate::utils;
use anyhow::{Context, Result};

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

fn get_leftist_rightist_groups(data: Vec<String>) -> (Vec<i128>, Vec<i128>) {
    data.into_iter()
        .filter_map(|line| {
            let mut nums = line
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse::<i128>().ok());
            match (nums.next(), nums.next()) {
                (Some(left), Some(right)) => Some((left, right)),
                _ => None,
            }
        })
        .unzip()
}

pub fn total_distance(data: Vec<String>) -> i128 {
    let (mut leftist, mut rightist) = get_leftist_rightist_groups(data);

    leftist.sort_unstable();
    rightist.sort_unstable();

    zip(leftist, rightist)
        .into_iter()
        .map(|(left, right)| (left - right).abs())
        .sum()
}

pub fn solve_part1() -> Result<String> {
    let input = utils::read_input(2024, 1).context("Could not read input file")?;

    let data = parse_input(&input).context("Could not parse input")?;

    let result = total_distance(data);

    Ok(result.to_string())
}

pub fn similarity_score(data: Vec<String>) -> i128 {
    let (leftist, rightist) = get_leftist_rightist_groups(data);

    let mut occurrence_table: HashMap<i128, i128> = HashMap::new();

    for location_id in rightist {
        *occurrence_table.entry(location_id).or_insert(0) += 1;
    }

    leftist
        .into_iter()
        .map(|location_id| location_id * occurrence_table.get(&location_id).unwrap_or(&0))
        .sum()
}

pub fn solve_part2() -> Result<String> {
    let input = utils::read_input(2024, 1).context("Could not read input file")?;

    let data = parse_input(&input).context("Could not parse input")?;

    let result = similarity_score(data);

    Ok(result.to_string())
}
