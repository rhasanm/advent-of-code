use crate::utils::{prelude::read_input, Grid};
use anyhow::Result;

pub const CORRUPTED_BYTE: char = '#';
pub const SAFE_BYTE: char = '.';
pub const VISITED_BYTE: char = 'O';

pub type BytePosition = (i32, i32);
pub struct Memory {
    pub space: Grid,
}

impl Memory {
    pub fn new(rows: i32, cols: i32, fill: char) -> Self {
        Self {
            space: Grid::with_dimensions(rows, cols, fill),
        }
    }

    pub fn mark_corrupted_bytes(&mut self, fallen_bytes: Vec<BytePosition>) -> Result<()> {
        fallen_bytes.iter().for_each(|&(x, y)| {
            self.space.set(x, y, CORRUPTED_BYTE);
        });
        Ok(())
    }
}

pub fn parse_input(input: &str) -> Result<Vec<BytePosition>> {
    Ok(input
        .lines()
        .map(String::from)
        .map(|line| {
            line.split(',')
                .map(|pos| pos.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|pos| (pos[1], pos[0]))
        .collect())
}

pub fn solve_part1() -> Result<String> {
    let input = read_input(2024, 18)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 18)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
