use std::collections::HashMap;

use crate::utils::prelude::read_input;
use anyhow::{Ok, Result};

#[derive(Debug, PartialEq)]
pub enum BooleanOperation {
    XOR,
    AND,
    OR,
}

pub type Wire = String;
pub type WireValueMap = HashMap<String, i8>;

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub operation_type: Option<BooleanOperation>,
    pub input1: Wire,
    pub input2: Wire,
    pub output: Wire,
}

impl Operation {
    pub fn new(
        input1: Wire,
        input2: Wire,
        operation_type: Option<BooleanOperation>,
        output: Wire,
    ) -> Self {
        Self {
            operation_type,
            input1,
            input2,
            output,
        }
    }
}

pub fn parse_input(input: &str) -> Result<(WireValueMap, Vec<Operation>)> {
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let starting_values: WireValueMap = lines
        .iter()
        .take_while(|&line| line != "")
        .map(|line| line.split(':').collect::<Vec<_>>())
        .map(|values| {
            (
                values[0].to_string(),
                values[1].trim().parse::<i8>().unwrap(),
            )
        })
        .collect();

    let operations: Vec<Operation> = lines
        .iter()
        .rev()
        .take_while(|&line| line != "")
        .map(|line| line.split("->").collect::<Vec<_>>())
        .map(|values| {
            let output = values[1].trim().to_string();
            let input: Vec<&str> = values[0].split_whitespace().collect();
            Operation::new(
                input[0].to_string(),
                input[2].to_string(),
                match input[1] {
                    "XOR" => Some(BooleanOperation::XOR),
                    "OR" => Some(BooleanOperation::OR),
                    "AND" => Some(BooleanOperation::AND),
                    _ => None,
                },
                output,
            )
        })
        .collect();

    Ok((starting_values, operations))
}

pub fn map_reduce(initial_values: &mut WireValueMap, operations: &mut Vec<Operation>) -> Result<i64> {
    while !operations.is_empty() {
        let operations_to_process: Vec<(usize, &Operation)> = operations
            .iter()
            .enumerate()
            .filter(|(_, operation)| {
                initial_values.contains_key(&operation.input1) && 
                initial_values.contains_key(&operation.input2)
            })
            .collect();

        let indices_to_remove: Vec<usize> = operations_to_process
            .iter()
            .map(|&(idx, operation)| {
                let &input1 = initial_values.get(&operation.input1).unwrap();
                let &input2 = initial_values.get(&operation.input2).unwrap();
                let result = match &operation.operation_type {
                    Some(BooleanOperation::XOR) => input1 ^ input2,
                    Some(BooleanOperation::AND) => input1 & input2,
                    Some(BooleanOperation::OR) => input1 | input2,
                    None => todo!(),
                };
                initial_values.insert(operation.output.clone(), result);
                idx
            })
            .collect();
        
        for idx in indices_to_remove.iter().rev() {
            operations.remove(*idx);
        }
    }

    let mut binaries = initial_values
    .iter()
    .filter_map(|(k, &v)| k.strip_prefix('z').map(|num| (num, v)))
    .collect::<Vec<_>>();
    binaries.sort();
    binaries.reverse();
    
    let binary_string = binaries
    .iter()
    .map(|(_, v)| v.to_string())
    .collect::<String>();

    Ok(i64::from_str_radix(&binary_string, 2)?)
}

pub fn solve_part1() -> Result<i64> {
    let input = read_input(2024, 24)?;
    let (mut initial_values, mut operations) = parse_input(&input)?;

    let output = map_reduce(&mut initial_values, &mut operations)?;

    Ok(output)
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 24)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
