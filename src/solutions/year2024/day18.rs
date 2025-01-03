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

    pub fn find_shortest_path_to_exit(&mut self, exit: BytePosition) -> Result<(i32, Vec<BytePosition>)> {
        let mut bytes: VecDeque<Props> = VecDeque::new();
        bytes.push_back(((0, 0), 0));
    
        let mut parents: Vec<Vec<Option<BytePosition>>> = vec![vec![None; self.space.cols as usize]; self.space.rows as usize];
    
        while let Some(prop) = bytes.pop_front() {
            if prop.0 == exit {
                let mut path = Vec::new();
                let mut current = exit;
                while current != (0, 0) {
                    path.push(current);
                    current = parents[current.0 as usize][current.1 as usize].unwrap();
                }
                path.push((0, 0));
                path.reverse();
                return Ok((prop.1, path));
            }
    
            for (dx, dy) in CARDINAL_DIRECTIONS {
                let x = dx + prop.0.0;
                let y = dy + prop.0.1;
    
                if self.space.in_bounds(x, y) && self.space.get(x, y) == Some(SAFE_BYTE) {
                    bytes.push_back(((x, y), prop.1 + 1));
                    self.space.set(x, y, VISITED_BYTE);
                    parents[x as usize][y as usize] = Some(prop.0);
                }
            }
        }
        Ok((-1, Vec::new()))
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
    let _ = memory.mark_corrupted_bytes(data.iter().take(1024).cloned().collect::<Vec<BytePosition>>());

    println!("{}", memory.space);

    let (steps, path) = memory.find_shortest_path_to_exit((70, 70))?;

    println!("{}", memory.space);

    crate::utils::visualization::print_colored_grid(&memory.space.cells);

    let _ = crate::utils::visualization::render_grid_as_image(
        &memory.space.cells,
        &path,
        "outputs/2024/day18/grid.png",
    );

    let _ = crate::utils::visualization::render_grid_interactive(&memory.space.cells);

    Ok(steps)
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 18)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
