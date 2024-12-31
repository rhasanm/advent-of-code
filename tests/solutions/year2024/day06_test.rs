use advent_of_code::solutions::year2024::day06;
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_mapper_object() {
    let direction = day06::MAPPER.get(&'>').unwrap();

    assert_eq!(direction, &(0, 1));
}

#[test]
fn test_get_guards_location() {
    let data = day06::parse_input(EXAMPLE_INPUT).unwrap();
    let location = day06::get_guards_location(&data).unwrap();

    assert_eq!(location.0, '^');
    assert_eq!(location.1, (6, 4));
}

#[test]
fn test_part1_example() -> Result<()> {
    let data = day06::parse_input(EXAMPLE_INPUT)?;

    let visited = day06::walk(data);
    assert_eq!(visited, 41);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day06::solve_part1()?;
    println!("Solution Part 1: {}", solution);
    assert_eq!(solution, 4973);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day06::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day06::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
