use advent_of_code::solutions::year2024::day11;
use anyhow::Result;
use rayon::vec;

const EXAMPLE_INPUT: &str = "125 17";

#[test]
fn test_blink() {
    let input = day11::parse_input(EXAMPLE_INPUT).unwrap();

    let stones = day11::blink(input);
    assert_eq!(stones, vec![253000, 1, 7]);

    let next_blink = day11::blink(stones);
    assert_eq!(next_blink, vec![253, 0, 2024, 14168])
}

#[test]
fn test_part1_example() -> Result<()> {
    let mut stones = day11::parse_input(EXAMPLE_INPUT)?;

    for _ in 0..25 {
        stones = day11::blink(stones);
    }

    assert_eq!(stones.len(), 55312);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day11::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 228668);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day11::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day11::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
