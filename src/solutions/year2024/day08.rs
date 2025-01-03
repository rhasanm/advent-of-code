use std::collections::HashMap;

use crate::utils::{prelude::read_input, Grid};
use anyhow::{Ok, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: -1, y: -1 }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn distance(&self, other: &Point) -> Point {
        Point {
            x: (self.x - other.x).abs(),
            y: (self.y - other.y).abs(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
    d: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            p1: p1,
            p2: p2,
            d: p1.distance(&p2),
        }
    }

    fn calculate_antinodes(&self) -> (Point, Point) {
        if self.p1.y < self.p2.y {
            (
                Point::new(self.p1.x - self.d.x, self.p1.y - self.d.y),
                Point::new(self.p2.x + self.d.x, self.p2.y + self.d.y),
            )
        } else {
            (
                Point::new(self.p1.x - self.d.x, self.p1.y + self.d.y),
                Point::new(self.p2.x + self.d.x, self.p2.y - self.d.y),
            )
        }
    }
}

pub struct AntennasGrid {
    grid: Grid,
}

impl AntennasGrid {
    pub fn new(data: Vec<String>) -> Self {
        Self {
            grid: Grid::new(&data),
        }
    }

    pub fn count_antinodes(&self) -> usize {
        self.grid
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&ch| ch == '#')
            .count()
    }
}

impl std::ops::Deref for AntennasGrid {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl std::ops::DerefMut for AntennasGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

pub struct City {
    pub grid: AntennasGrid,
}

impl City {
    pub fn new(data: Vec<String>) -> Self {
        Self {
            grid: AntennasGrid::new(data),
        }
    }

    pub fn find_lines(&self) -> Vec<Line> {
        let mut antenna_locations: HashMap<char, Vec<Point>> = HashMap::new();
        let mut lines = Vec::new();

        for x in 0..self.grid.rows as i32 {
            for y in 0..self.grid.cols as i32 {
                if let Some(ch) = self.grid.get(x, y) {
                    if ch != '.' {
                        if let Some(existing_points) = antenna_locations.get(&ch) {
                            for &start in existing_points {
                                lines.push(Line::new(start, Point::new(x, y)));
                            }
                        }
                        antenna_locations
                            .entry(ch)
                            .or_default()
                            .push(Point::new(x, y));
                    }
                }
            }
        }
        lines
    }

    pub fn mark_antinodes(&mut self, lines: &[Line]) -> Result<()> {
        for line in lines {
            let (t1, t2) = line.calculate_antinodes();

            if self.grid.in_bounds(t1.x, t1.y) {
                self.grid.set(t1.x, t1.y, '#');
            }
            if self.grid.in_bounds(t2.x, t2.y) {
                self.grid.set(t2.x, t2.y, '#');
            }
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn solve_part1() -> Result<usize> {
    let input = read_input(2024, 8)?;
    let data = parse_input(&input)?;

    let mut city = City::new(data);
    let lines = city.find_lines();

    city.mark_antinodes(&lines)?;
    Ok(city.grid.count_antinodes())
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 8)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
