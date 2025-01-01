use advent_of_code::{
    solutions::year2024::day10::{self, trailheads_on_topographic_map},
    utils::Grid,
};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn test_trailheads_on_topographic_map() -> Result<()> {
    // let data = day10::parse_input("\
    // 001
    // 100
    // 101")?;

    // let grid = Grid::new(&data);
    // let trailheads = trailheads_on_topographic_map(&grid)?;

    // assert_eq!(trailheads, 0);

    let data = day10::parse_input(
        "\
    0123
    1234
    8765
    9876",
    )?;

    let grid = Grid::new(&data);
    let trailheads = trailheads_on_topographic_map(&grid)?;

    assert_eq!(trailheads, 1);

    Ok(())
}

#[test]
fn test_part1_example() -> Result<()> {
    let data = day10::parse_input(EXAMPLE_INPUT)?;

    let grid = Grid::new(&data);
    let trailheads = trailheads_on_topographic_map(&grid)?;

    assert_eq!(trailheads, 36);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day10::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 688);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day10::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day10::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
