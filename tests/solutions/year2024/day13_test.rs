use advent_of_code::solutions::year2024::day13::{self, find_combination, Input};
use anyhow::{Ok, Result};

const EXAMPLE_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[test]
fn test_button_configurations() -> Result<()> {
    let input = day13::parse_input(
        "\
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400
    ",
    )
    .unwrap();
    let configuration = day13::button_configurations(input);

    assert_eq!(
        configuration.unwrap(),
        vec![Input::new(94, 22, 34, 67, 8400, 5400)]
    );
    Ok(())
}

#[test]
fn test_find_combinations() -> Result<()> {
    let combinations = day13::find_combination(&Input::new(94, 22, 34, 67, 8400, 5400)).unwrap();

    assert_eq!(combinations, (80, 40));

    let combinations = day13::find_combination(&Input::new(17, 84, 86, 37, 7870, 6450)).unwrap();

    assert_eq!(combinations, (38, 86));

    let combinations = day13::find_combination(&Input::new(69, 27, 23, 71, 18641, 10279));

    assert_eq!(combinations.unwrap_err().to_string(), "No solution found");
    Ok(())
}

#[test]
fn test_part1_example() -> Result<()> {
    let data = day13::parse_input(EXAMPLE_INPUT)?;

    let configurations = day13::button_configurations(data).unwrap();
    let tokens: i32 = configurations
        .iter()
        .map(|config| find_combination(config).unwrap_or((0, 0)))
        .map(|(a, b)| a * 3 + b)
        .sum();

    assert_eq!(tokens, 480);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day13::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 33481);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day13::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day13::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
