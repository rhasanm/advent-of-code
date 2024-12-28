use std::collections::{HashMap, VecDeque};

use crate::utils;
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn inside_farm(limit_x: i32, limit_y: i32, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < limit_x && y < limit_y
}

pub fn flood_fill(arrangement: &mut Vec<String>, x: usize, y: usize) -> (char, i128) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut q = VecDeque::<(usize, usize)>::from([(x, y)]);
    let plant = arrangement[x].chars().nth(y).unwrap();
    arrangement[x].replace_range(y..=y, "#");
    let mut area = 0;
    let mut perimeter = 0;

    while !q.is_empty() {
        let grid = q.pop_front().unwrap();
        area += 1;
        
        for (dx, dy) in directions {
            let next_x = grid.0 as i32 + dx;
            let next_y = grid.1 as i32 + dy;

            if inside_farm(
                arrangement.len() as i32,
                arrangement[0].len() as i32,
                next_x,
                next_y,
            ) && arrangement[next_x as usize]
                .chars()
                .nth(next_y as usize)
                .unwrap()
                == plant
            {
                arrangement[next_x as usize].replace_range(next_y as usize..=next_y as usize, "#");

                q.push_back((next_x as usize, next_y as usize));
            } else if !inside_farm(
                arrangement.len() as i32,
                arrangement[0].len() as i32,
                next_x,
                next_y,
            ) || (inside_farm(
                arrangement.len() as i32,
                arrangement[0].len() as i32,
                next_x,
                next_y,
            ) && arrangement[next_x as usize]
                .chars()
                .nth(next_y as usize)
                .unwrap()
                != '#')
            {
                perimeter += 1;
            }
        }
    }

    *arrangement = arrangement.iter().map(|line| line.replace("#", ".")).collect();

    (plant, area * perimeter)
}

pub fn calculate_fencing_price(arrangement: &mut Vec<String>) -> i128 {
    let mut memo = HashMap::new();

    for row in 0..arrangement.len() {
        for col in 0..arrangement[0].len() {
            if arrangement[row].chars().nth(col).unwrap() != '.' {
                let (plant, cost) = flood_fill(arrangement, row, col);

                *memo.entry(plant).or_insert(0) += cost;
            }
        }
    }

    println!("{:?}", memo);
    memo.values().sum()
}

pub fn solve_part1() -> Result<i128> {
    let input = utils::read_input(2024, 12)?;
    let mut data = parse_input(&input)?;

    let cost = calculate_fencing_price(&mut data);
    Ok(cost)
}

pub fn solve_part2() -> Result<String> {
    let input = utils::read_input(2024, 12)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
