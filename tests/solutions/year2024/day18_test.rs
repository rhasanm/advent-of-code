use advent_of_code::solutions::year2024::day18::{self, BytePosition, Memory, SAFE_BYTE};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day18::parse_input(EXAMPLE_INPUT)?;

    let mut memory = Memory::new(7, 7, SAFE_BYTE);
    let _ =
        memory.mark_corrupted_bytes(data.iter().take(12).cloned().collect::<Vec<BytePosition>>());

    println!("{}", memory.space);

    let steps = memory.find_shortest_path_to_exit((6, 6))?;

    println!("{}", memory.space);

    assert_eq!(steps, 22);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day18::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 262);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day18::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day18::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
