use std::collections::HashMap;

use crate::{
    common::prelude::CARDINAL_DIRECTIONS,
    utils::{prelude::read_input, Grid},
};
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(|line| line.trim().to_string()).collect())
}

pub fn recur_all(grid: &Grid, x: i32, y: i32) -> i128 {
    let current_height = match grid.get(x, y) {
        Some(height) => height,
        None => return 0,
    };

    if current_height == '9' {
        return 1;
    }

    let mut result = 0;
    for (dx, dy) in CARDINAL_DIRECTIONS {
        if let Some(next_scale) = grid.get(x + dx, y + dy) {
            if next_scale as u8 == current_height as u8 + 1 {
                result += recur_all(grid, x + dx, y + dy);
            }
        }
    }
    result
}

pub fn recur(grid: &Grid, x: i32, y: i32, visited: &mut HashMap<(i32, i32), bool>) -> i128 {
    let current_height = match grid.get(x, y) {
        Some(height) => height,
        None => return 0,
    };

    if current_height == '9' {
        return if visited.contains_key(&(x, y)) {
            0
        } else {
            visited.insert((x, y), true);
            1
        };
    }

    let mut result = 0;
    for (dx, dy) in CARDINAL_DIRECTIONS {
        if let Some(next_scale) = grid.get(x + dx, y + dy) {
            if next_scale as u8 == current_height as u8 + 1 {
                result += recur(grid, x + dx, y + dy, visited);
            }
        }
    }
    result
}

pub fn trailheads_on_topographic_map(grid: &Grid) -> Result<i128> {
    let mut total = 0;

    for (row_idx, row) in grid.cells.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '0' {
                total += recur(grid, row_idx as i32, col_idx as i32, &mut HashMap::new());
            }
        }
    }

    Ok(total)
}

pub fn trailheads_on_topographic_map_all(grid: &Grid) -> Result<i128> {
    let mut total = 0;

    for (row_idx, row) in grid.cells.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == '0' {
                total += recur_all(grid, row_idx as i32, col_idx as i32);
            }
        }
    }

    Ok(total)
}

pub fn solve_part1() -> Result<i128> {
    let input = read_input(2024, 10)?;
    let data = parse_input(&input)?;

    let grid = Grid::new(&data);
    let trailheads = trailheads_on_topographic_map(&grid)?;

    Ok(trailheads)
}

pub fn solve_part2() -> Result<i128> {
    let input = read_input(2024, 10)?;
    let data = parse_input(&input)?;

    let grid = Grid::new(&data);
    let trailheads = trailheads_on_topographic_map_all(&grid)?;

    Ok(trailheads)
}
