use std::collections::VecDeque;

use crate::{common::prelude::CARDINAL_DIRECTIONS, utils::{prelude::read_input, Grid}};
use anyhow::{Ok, Result};

pub const CORRUPTED_BYTE: char = '#';
pub const SAFE_BYTE: char = '.';
pub const VISITED_BYTE: char = 'O';

pub type BytePosition = (i32, i32);
pub type Props = (BytePosition, i32);
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

    pub fn find_shortest_path_to_exit(&mut self, exit: BytePosition) -> Result<i32> {
        let mut bytes: VecDeque<Props> = VecDeque::new();
        bytes.push_back(((0, 0), 0));

        while let Some(prop) = bytes.pop_front() {
            if prop.0 == exit {
                return Ok(prop.1);
            }

            for (dx, dy) in CARDINAL_DIRECTIONS {
                let x = dx + prop.0.0;
                let y = dy + prop.0.1;

                if self.space.in_bounds(x, y) && self.space.get(x, y) == Some(SAFE_BYTE) {
                    bytes.push_back(((x, y), prop.1 + 1));
                    self.space.set(x, y, VISITED_BYTE);
                }
            }
        }
        Ok(-1)
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

pub fn solve_part1() -> Result<i32> {
    let input = read_input(2024, 18)?;
    let data = parse_input(&input)?;

    let mut memory = Memory::new(71, 71, SAFE_BYTE);
    let _ =
        memory.mark_corrupted_bytes(data.iter().take(1024).cloned().collect::<Vec<BytePosition>>());

    println!("{}", memory.space);

    let steps = memory.find_shortest_path_to_exit((70, 70))?;

    println!("{}", memory.space);

    Ok(steps)
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 18)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
