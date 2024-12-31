use crate::utils::prelude::read_input;
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<String> {
    Ok(input.trim().to_string())
}

pub fn represent_blocks(layout: &str) -> Vec<i32> {
    let mut blocks = Vec::new();
    let mut file_id = 0;

    for (idx, ch) in layout.chars().enumerate() {
        let count = ch.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            blocks.extend(vec![file_id; count]);
            file_id += 1;
        } else {
            blocks.extend(vec![-1; count]);
        }
    }
    blocks
}

pub fn apply_amphipod(mut blocks: Vec<i32>) -> Vec<i32> {
    loop {
        let last_filebit = blocks.iter().rposition(|&x| x != -1).unwrap();
        let first_freebit = blocks.iter().position(|&x| x == -1).unwrap();

        if last_filebit <= first_freebit {
            break;
        }

        blocks[first_freebit] = blocks[last_filebit];
        blocks[last_filebit] = -1;
    }
    blocks
}

pub fn calculate_checksum(blocks: &[i32]) -> u128 {
    blocks
        .iter()
        .enumerate()
        .filter(|&(_, &id)| id != -1)
        .map(|(pos, &id)| pos as u128 * id as u128)
        .sum()
}

pub fn solve_part1() -> Result<u128> {
    let input = read_input(2024, 9)?;
    let data = parse_input(&input)?;

    let mut blocks = represent_blocks(&data);
    blocks = apply_amphipod(blocks);

    Ok(calculate_checksum(&blocks))
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 9)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
