use crate::utils;
use anyhow::{Ok, Result};

pub struct Input {
    pub test_value: i64,
    pub operands: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Plus,
    Multiplication,
}

pub fn parse_input(input: &str) -> Result<Vec<Input>> {
    input
        .lines()
        .map(|line| {
            let (test_part, numbers_part) = line.split_once(':').unwrap();

            let test_value = test_part.trim().parse()?;

            let operands: Vec<i64> = numbers_part
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            Ok(Input {
                test_value,
                operands,
            })
        })
        .collect()
}

pub fn find_combination(equation: &Input, operators: Vec<Operators>) -> Result<bool> {
    if operators.len() == equation.operands.len() - 1 {
        let mut current_operator = 0;
        let test_value = equation.operands.iter().try_fold(0i64, |acc, &x| {
            if acc == 0 {
                return acc.checked_add(x);
            }

            match operators.get(current_operator).unwrap() {
                Operators::Plus => {
                    current_operator += 1;
                    acc.checked_add(x)
                }
                Operators::Multiplication => {
                    current_operator += 1;
                    acc.checked_mul(x)
                }
            }
        });
        return Ok(test_value.unwrap() == equation.test_value);
    }

    let mut plus_appended = operators.clone();
    plus_appended.push(Operators::Plus);
    let plus = find_combination(equation, plus_appended)?;

    let mut mul_appended = operators.clone();
    mul_appended.push(Operators::Multiplication);
    let mul = find_combination(equation, mul_appended)?;

    Ok(plus || mul)
}

pub fn solve_part1() -> Result<i64> {
    let input = utils::read_input(2024, 7)?;
    let data = parse_input(&input)?;

    let total_calibration: i64 = data
        .iter()
        .filter(|&equation| find_combination(equation, vec![]).unwrap())
        .map(|equation| equation.test_value)
        .sum();

    Ok(total_calibration)
}

pub fn solve_part2() -> Result<String> {
    let input = utils::read_input(2024, 7)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
