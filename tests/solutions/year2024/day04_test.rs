use advent_of_code::solutions::year2024::day04;
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day04::parse_input(EXAMPLE_INPUT)?;

    let possible = day04::count_xmas(&data);
    assert_eq!(possible, 18);

    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day04::solve_part1()?;
    println!("Solution Part 1: {}", solution);
    assert_eq!(solution, 2468);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day04::parse_input(EXAMPLE_INPUT)?;

    let solution = day04::count_x(&data);

    assert_eq!(solution, 9);

    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day04::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    assert_eq!(solution, 1864);
    Ok(())
}
