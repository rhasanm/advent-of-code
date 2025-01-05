use advent_of_code::solutions::year2024::day24::{self, map_reduce, BooleanOperation, Operation};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[test]
fn test_part1_example() -> Result<()> {
    let (mut initial_values, mut operations) = day24::parse_input(EXAMPLE_INPUT)?;

    assert_eq!(initial_values.get("x00").unwrap(), &1);
    assert_eq!(
        operations[0],
        Operation::new(
            "tnw".to_string(),
            "pbm".to_string(),
            Some(BooleanOperation::OR),
            "gnj".to_string()
        )
    );

    let output = map_reduce(&mut initial_values, &mut operations)?;
    assert_eq!(output, 2024);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day24::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 56729630917616);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day24::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day24::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
