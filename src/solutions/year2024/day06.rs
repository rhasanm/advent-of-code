use std::collections::HashMap;

use crate::utils;
use anyhow::Result;

lazy_static::lazy_static! {
    pub static ref MAPPER: HashMap<char, (i128, i128)> = {
        let mut m = HashMap::new();
        m.insert('v', (1, 0));
        m.insert('>', (0, 1));
        m.insert('^', (-1, 0));
        m.insert('<', (0, -1));
        m
    };

    pub static ref DIRECTIONS: Vec<char> = vec!['>', '<', 'v', '^'];
}

fn is_within_bounds(position: (i128, i128), path: &Vec<String>) -> bool {
    let (x, y) = position;
    x >= 0 && y >= 0 && (x as usize) < path.len() && (y as usize) < path[0].len()
}

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn get_guards_location(path: &Vec<String>) -> Option<(char, (i128, i128))> {
    path.iter().enumerate().find_map(|(i, line)| {
        line.chars().enumerate().find_map(|(j, object)| {
            if object != '.' && object != '#' {
                Some((object, (i as i128, j as i128)))
            } else {
                None
            }
        })
    })
}

pub fn walk(mut path: Vec<String>) -> i128 {
    let (mut direction, mut guard) = get_guards_location(&path).unwrap();

    let mut row: Vec<char> = path[guard.0 as usize].chars().collect();
    row[guard.1 as usize] = 'X';
    path[guard.0 as usize] = row.into_iter().collect();

    let mut visited = 0;
    loop {
        if !is_within_bounds(guard, &path) {
            break;
        }

        let next_move = *MAPPER.get(&direction).unwrap();
        let next_square = (guard.0 + next_move.0, guard.1 + next_move.1);
        if !is_within_bounds(next_square, &path) {
            visited += 1;
            break;
        }
        let next_square_el = path[next_square.0 as usize]
            .chars()
            .nth(next_square.1 as usize)
            .unwrap();

        if next_square_el == '#' {
            direction = match direction {
                '<' => '^',
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                _ => direction,
            };
            continue;
        }
        if next_square_el == '.' {
            let mut row: Vec<char> = path[next_square.0 as usize].chars().collect();
            row[next_square.1 as usize] = 'X';
            visited += 1;
            path[next_square.0 as usize] = row.into_iter().collect();
        }

        guard = next_square
    }

    visited
}

pub fn solve_part1() -> Result<i128> {
    let input = utils::read_input(2024, 6)?;
    let data = parse_input(&input)?;

    let visited = walk(data);
    Ok(visited)
}

pub fn solve_part2() -> Result<String> {
    let input = utils::read_input(2024, 6)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
