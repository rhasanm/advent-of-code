use advent_of_code::solutions::year2024::day08::{self, City};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day08::parse_input(EXAMPLE_INPUT)?;

    let mut city = City::new(data);
    let lines = city.find_lines();

    city.mark_antinodes(&lines)?;
    let antinodes = city.grid.count_antinodes();

    assert_eq!(antinodes, 14);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day08::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 364);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day08::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day08::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
