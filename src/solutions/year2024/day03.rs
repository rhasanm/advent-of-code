use crate::utils::prelude::read_input;
use anyhow::{Context, Result};
use regex::Regex;

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

fn valid_mul_instructions(data: Vec<String>) -> Vec<String> {
    static INSTRUCTION_PATTERN: &str = r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)";
    let re = Regex::new(INSTRUCTION_PATTERN).expect("Invalid regex pattern");

    let mut enabled = true;
    data.iter()
        .flat_map(|line| re.find_iter(line).map(|m| m.as_str().to_string()))
        .filter_map(|instruction| match instruction.as_str() {
            "don't()" => {
                enabled = false;
                None
            }
            "do()" => {
                enabled = true;
                None
            }
            _ if enabled => Some(instruction),
            _ => None,
        })
        .collect()
}

fn mul_instructions(data: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    data.iter()
        .flat_map(|instruction| {
            re.find_iter(instruction)
                .map(|m| m.as_str().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn add_up_all_after_mul(instructions: Vec<String>) -> i128 {
    let re = Regex::new(r"\d+").unwrap();
    instructions
        .iter()
        .map(|instruction| {
            let numbers: Vec<i128> = re
                .find_iter(instruction)
                .flat_map(|number| number.as_str().parse::<i128>())
                .collect();

            numbers[0] * numbers[1]
        })
        .sum()
}

pub fn solve_part1() -> Result<String> {
    let input = read_input(2024, 3).context("Could not read input file")?;

    let data = parse_input(&input).context("Could not parse input")?;

    let instructions = mul_instructions(data);

    let result = add_up_all_after_mul(instructions);

    Ok(result.to_string())
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 3).context("Could not read input file")?;

    let data = parse_input(&input).context("Could not parse input")?;

    let instructions = valid_mul_instructions(data);

    let result = add_up_all_after_mul(instructions);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use regex::Regex;

    use crate::solutions::year2024::{
        day03::solve_part2,
        day03::{
            self, add_up_all_after_mul, mul_instructions, solve_part1, valid_mul_instructions,
        },
    };

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_mul_instructions() {
        println!(
            "{:?}",
            mul_instructions(vec![
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            ])
        );
        assert!(true, "This is not valid");
    }

    #[test]
    fn test_part1_example() -> Result<()> {
        let data = day03::parse_input(EXAMPLE_INPUT)?;

        let instructions = mul_instructions(data);

        let result = add_up_all_after_mul(instructions);

        assert_eq!(result, 161);
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<()> {
        let data = day03::parse_input(EXAMPLE_INPUT2)?;

        let instructions = valid_mul_instructions(data);

        let result = add_up_all_after_mul(instructions);

        assert_eq!(result, 48);
        Ok(())
    }

    #[test]
    fn test_solve_part1() {
        let res = solve_part1().unwrap();

        assert_eq!(res, "161289189")
    }

    #[test]
    fn test_solve_part2() {
        let res = solve_part2().unwrap();

        assert_eq!(res, "83595109")
    }

    #[test]
    fn test_number_conversion() {
        let re = Regex::new(r"\d+").unwrap();
        let instruction = "mul(212,114)";

        let numbers: Vec<i128> = re
            .find_iter(instruction)
            .flat_map(|number| number.as_str().parse::<i128>())
            .collect();

        println!("{:?}", numbers);
    }
}
