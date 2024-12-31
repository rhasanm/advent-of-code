use std::collections::{HashMap, VecDeque};

use crate::utils::prelude::{read_input, Grid};
use anyhow::Result;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const VISITED: char = '#';
const EMPTY: char = '.';

pub fn flood_fill(grid: &mut Grid, start_x: usize, start_y: usize) -> (char, i128) {
    let target = grid.get(start_x as i32, start_y as i32).unwrap();
    if target == EMPTY {
        return (EMPTY, 0);
    }

    let mut queue = VecDeque::from([(start_x as i32, start_y as i32)]);
    grid.set(start_x as i32, start_y as i32, VISITED);

    let mut area = 0;
    let mut perimeter = 0;

    while let Some((x, y)) = queue.pop_front() {
        area += 1;

        for &(dx, dy) in &DIRECTIONS {
            let (next_x, next_y) = (x + dx, y + dy);

            match grid.get(next_x, next_y) {
                Some(cell) if cell == target => {
                    grid.set(next_x, next_y, VISITED);
                    queue.push_back((next_x, next_y));
                }
                Some(cell) if cell != VISITED => perimeter += 1,
                None => perimeter += 1,
                _ => {}
            }
        }
    }

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.get(row, col) == Some(VISITED) {
                grid.set(row, col, EMPTY);
            }
        }
    }

    (target, area * perimeter)
}

pub fn calculate_fencing_cost(arrangement: &mut Vec<String>) -> Result<i128> {
    let mut grid = Grid::new(&arrangement);
    let mut costs: HashMap<char, i128> = HashMap::new();

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if let Some(cell) = grid.get(row, col) {
                if cell != EMPTY {
                    let (plant, cost) = flood_fill(&mut grid, row as usize, col as usize);
                    *costs.entry(plant).or_default() += cost;
                }
            }
        }
    }

    Ok(costs.values().sum())
}

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn solve_part1() -> Result<i128> {
    let input = read_input(2024, 12)?;
    let mut data = parse_input(&input)?;

    let cost = calculate_fencing_cost(&mut data).unwrap();
    Ok(cost)
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 12)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
