use crate::utils;
use anyhow::Result;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn check_direction(grid: &[String], i: usize, j: usize, di: isize, dj: isize) -> bool {
    let target = ['X', 'M', 'A', 'S'];

    for k in 0..4 {
        let new_i = i as isize + di * k as isize;
        let new_j = j as isize + dj * k as isize;

        if new_i < 0 || new_j < 0 || new_i >= grid.len() as isize || new_j >= grid[0].len() as isize
        {
            return false;
        }

        if grid[new_i as usize].chars().nth(new_j as usize).unwrap() != target[k] {
            return false;
        }
    }
    true
}

pub fn count_x(grid: &[String]) -> i128 {
    let directions_lr = [(-1, -1), (1, 1)];
    let directions_rl = [(1, -1), (-1, 1)];

    let is_in_bounds = |x: isize, y: isize| -> bool {
        x >= 0 && y >= 0 && x < grid.len() as isize && y < grid[0].len() as isize
    };

    let collect_chars = |i: usize, j: usize, directions: &[(isize, isize)]| -> HashSet<char> {
        directions
            .iter()
            .filter_map(|&(dx, dy)| {
                let new_i = i as isize + dx;
                let new_j = j as isize + dy;
                if is_in_bounds(new_i, new_j) {
                    Some(grid[new_i as usize].chars().nth(new_j as usize).unwrap())
                } else {
                    None
                }
            })
            .collect()
    };

    let target: HashSet<char> = ['M', 'S'].into();

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter(|&(j, c)| {
                    c == 'A'
                        && collect_chars(i, j, &directions_lr) == target
                        && collect_chars(i, j, &directions_rl) == target
                })
                .count() as i128
        })
        .sum()
}

pub fn count_xmas(grid: &[String]) -> i128 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i].chars().nth(j).unwrap() == 'X' {
                for &(di, dj) in &directions {
                    if check_direction(grid, i, j, di, dj) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn solve_part1() -> Result<i128> {
    let input = utils::read_input(2024, 4)?;
    let data = parse_input(&input)?;

    let xmas_count = count_xmas(&data);

    Ok(xmas_count)
}

pub fn solve_part2() -> Result<i128> {
    let input = utils::read_input(2024, 4)?;
    let data = parse_input(&input)?;

    let x_count = count_x(&data);

    Ok(x_count)
}
