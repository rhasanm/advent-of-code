use advent_of_code::{solutions::year2024::day07, utils::prelude::read_input};

pub fn benchmark_day07_part1() -> i64 {
    let input = read_input(2024, 7).unwrap();
    let data = day07::parse_input::<i64>(&input).unwrap();
    iai::black_box(
        data.iter()
            .filter(|&equation| day07::find_combination_using_binary(equation).unwrap())
            .map(|equation| equation.test_value)
            .sum()
    )
}

pub fn benchmark_day07_part2() -> i128 {
    let input = read_input(2024, 7).unwrap();
    let data = day07::parse_input::<i128>(&input).unwrap();
    iai::black_box(
        data.iter()
            .filter(|&equation| day07::find_combination_with_concatenating(equation, vec![]).unwrap())
            .map(|equation| equation.test_value)
            .sum()
    )
}