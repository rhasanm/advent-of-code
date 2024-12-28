use crate::utils;
use anyhow::{Ok, Result};

#[derive(Debug, PartialEq)]
pub struct Input {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    t1: i32,
    t2: i32
}

impl Input {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32, t1: i32, t2: i32) -> Self {
        Self { x1, x2, y1, y2, t1, t2 }
    }
}

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(|line| line.trim().to_string()).collect())
}

// X * 94 + Y * 22 = X1
// X * 34 + Y * 67 = Y1
// 80*94 + 40*22 = 8400
// 80*34 + 40*67 = 5400

// X * 94 = 8400 - Y * 22
// X = (8400 - Y * 22) / 94 …. (1)

// X * 34 + Y * 67 = 5400

// ((8400 - Y * 22) / 94) * 34 + Y * 67 = 5400
// ((X1 - Y * y1) / x1) * x2 + Y * y2 = 5400
// fn binary_search()

pub fn find_combination(input: &Input) -> Result<(i32, i32)> {
    (1..=100)
    .flat_map(|a| (1..=100).map(move |b| (a, b)))
    .find(|&(a, b)| {
        let x_match = a * input.x1 + b * input.x2 == input.t1;
        let y_match = a * input.y1 + b * input.y2 == input.t2;
        x_match && y_match
    })
    .ok_or_else(|| anyhow::anyhow!("No valid combination found"))
}

pub fn button_configurations(input: Vec<String>) -> Result<Vec<Input>> {
    let mut result = Vec::new();
    let mut x1 = 0;
    let mut x2 = 0;
    let mut y1 = 0;
    let mut y2 = 0;
    let mut t1;
    let mut t2;

    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["Button", "A:", x, y] => {
                x1 = x.replace("X+", "").replace(",", "").parse()?;
                y1 = y.replace("Y+", "").parse()?;
            }
            ["Button", "B:", x, y] => {
                x2 = x.replace("X+", "").replace(",", "").parse()?;
                y2 = y.replace("Y+", "").parse()?;
            }
            ["Prize:", t1_str, t2_str] => {
                t1 = t1_str.split('=').nth(1).unwrap().replace(",", "").parse()?;
                t2 = t2_str.split('=').nth(1).unwrap().parse()?;
                result.push(Input::new(x1, x2, y1, y2, t1, t2));
            }
            _ => {}
        }
    }
    Ok(result)
}

pub fn solve_part1() -> Result<i32> {
    let input = utils::read_input(2024, 13)?;
    let data = parse_input(&input)?;
    
    let configurations = button_configurations(data).unwrap();
    let tokens: i32 = configurations
        .iter()
        .map(|config| find_combination(config).unwrap_or((0, 0)))
        .map(|(a, b)| a * 3 + b)
        .sum();

    Ok(tokens)
}

pub fn solve_part2() -> Result<String> {
    let input = utils::read_input(2024, 13)?;
    let data = parse_input(&input)?;
    
    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}