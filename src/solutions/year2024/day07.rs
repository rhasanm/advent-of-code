use std::{
    i128,
    str::FromStr,
};

use crate::common::Number;
use crate::utils::prelude::read_input;
use anyhow::{Ok, Result};

pub struct Input<T> {
    pub test_value: T,
    pub operands: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Plus,
    Multiplication,
    Concatenation
}

pub fn parse_input<T: Number>(input: &str) -> Result<Vec<Input<T>>>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    input
        .lines()
        .map(|line| {
            let (test_part, numbers_part) = line.split_once(':').unwrap();

            let test_value = test_part.trim().parse()?;

            let operands = numbers_part
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

pub fn find_combination_with_concatenating<T: Number>(equation: &Input<T>, operators: Vec<Operators>) -> Result<bool> {
    if operators.len() == equation.operands.len() - 1 {
        let mut current_operator = 0;
        let test_value = equation.operands.iter().try_fold(T::default(), |acc, x| {
            if acc == T::default() {
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
                },
                Operators::Concatenation => {
                    current_operator += 1;
                    let digits = (x.to_f64().unwrap().abs().log10().floor() as i32 + 1) as u32;
                    let concatenated = acc * T::from(10).unwrap().pow(digits) + *x;
                    Some(concatenated)
                }
            }
        });
        return Ok(test_value.unwrap() == equation.test_value);
    }

    let mut plus_appended = operators.clone();
    plus_appended.push(Operators::Plus);
    let plus = find_combination_with_concatenating(equation, plus_appended)?;

    let mut mul_appended = operators.clone();
    mul_appended.push(Operators::Multiplication);
    let mul = find_combination_with_concatenating(equation, mul_appended)?;

    let mut concatenated= operators.clone();
    concatenated.push(Operators::Concatenation);
    let con = find_combination_with_concatenating(equation, concatenated)?;

    Ok(plus || mul || con)
}

pub fn find_combination<T: Number>(equation: &Input<T>, operators: Vec<Operators>) -> Result<bool> {
    if operators.len() == equation.operands.len() - 1 {
        let mut current_operator = 0;
        let test_value = equation.operands.iter().try_fold(T::default(), |acc, x| {
            if acc == T::default() {
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
                },
                Operators::Concatenation => {
                    current_operator += 1;
                    let digits = (x.to_f64().unwrap().abs().log10().floor() as i32 + 1) as u32;
                    Some(acc * T::from(10).unwrap().pow(digits) + *x)
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
    let input = read_input(2024, 7)?;
    let data = parse_input::<i64>(&input)?;

    let total_calibration = data
        .iter()
        .filter(|&equation| find_combination(equation, vec![]).unwrap())
        .map(|equation| equation.test_value)
        .sum();

    Ok(total_calibration)
}

pub fn solve_part2() -> Result<i128> {
    let input = read_input(2024, 7)?;
    let data = parse_input::<i128>(&input)?;

    let total_calibration = data
        .iter()
        .filter(|&equation| find_combination_with_concatenating(equation, vec![]).unwrap())
        .map(|equation| equation.test_value)
        .sum();

    Ok(total_calibration)
}
