use advent_of_code::solutions::year2024::day09;
use anyhow::Result;

const EXAMPLE_INPUT: &str = "2333133121414131402";

#[test]
fn test_part1_example() -> Result<()> {
    let data = day09::parse_input(EXAMPLE_INPUT)?;

    let blocks = day09::represent_blocks(&data);

    fn blocks_to_string(blocks: &[i32]) -> String {
        blocks
            .iter()
            .map(|&id| {
                if id == -1 {
                    '.'
                } else {
                    char::from_digit(id as u32, 10).unwrap()
                }
            })
            .collect()
    }

    assert_eq!(
        blocks_to_string(&blocks),
        "00...111...2...333.44.5555.6666.777.888899"
    );

    let compacted = day09::apply_amphipod(blocks);
    assert_eq!(
        blocks_to_string(&compacted),
        "0099811188827773336446555566.............."
    );

    let checksum = day09::calculate_checksum(&compacted);
    assert_eq!(checksum, 1928);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day09::solve_part1()?;
    println!("Solution Part 1: {}", solution);
    assert_eq!(solution, 6291146824486);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day09::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day09::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
