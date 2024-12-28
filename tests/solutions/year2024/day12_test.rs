use advent_of_code::solutions::year2024::day12;
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn test_part1_example() -> Result<()> {
    let mut data = day12::parse_input(EXAMPLE_INPUT)?;
    
    let cost = day12::calculate_fencing_cost(&mut data).unwrap();
    assert_eq!(cost, 1930);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day12::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 1573474);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day12::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day12::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}