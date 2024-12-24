use anyhow::{Ok, Result};
use crate::utils;

type Data = Vec<Vec<i128>>;
#[derive(Debug)]
enum TrendType {
    Increasing,
    Decreasing,
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<i128>>> {
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let numbers: Vec<Vec<i128>> = lines
    .iter()
    .filter_map(|line| {
        let nums: Vec<i128> = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<i128>().ok())
            .collect();
        if nums.is_empty() {
            None
        } else {
            Some(nums)
        }
    })
    .collect();

    Ok(numbers)
}

pub fn check_safe_reports(data: Vec<Vec<i128>>) -> String {
    let candidates_after_differ_check: Vec<Vec<i128>> = data
        .iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|pair| {
                    let diff = (pair[0] - pair[1]).abs();
                    1 <= diff && diff <= 3
                })
        })
        .cloned()
        .collect();

    println!("{:?}", candidates_after_differ_check);

    let sds: Vec<Vec<i128>> = candidates_after_differ_check.iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|pair| {
                    pair[0] < pair[1]
                })
        })
        .cloned()
        .collect();

    let sis: Vec<Vec<i128>> = candidates_after_differ_check.iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|pair| {
                    pair[0] > pair[1]
                })
        })
        .cloned()
        .collect();

    (sis.len() + sds.len()).to_string()
}

pub fn solve_part1() -> Result<String> {
    let input = utils::read_input(2024, 02)?;
    let data = parse_input(&input)?;

    let tot = check_safe_reports(data);
    Ok(tot)
}

fn has_valid_differences(report: &[i128]) -> bool {
    report
        .windows(2)
        .all(|pair| {
            let diff = (pair[0] - pair[1]).abs();
            (1..=3).contains(&diff)
        })
}

fn has_trend(report: &[i128], trend_type: TrendType) -> bool {
    report
        .windows(2)
        .all(|pair| match trend_type {
            TrendType::Increasing => pair[0] < pair[1],
            TrendType::Decreasing => pair[0] > pair[1],
        })
}

fn is_unsafe(report: &[i128]) -> bool {
    !has_valid_differences(report) || 
    !(has_trend(report, TrendType::Increasing) || 
        has_trend(report, TrendType::Decreasing))
}

fn get_unsafe_reports(data: Data) -> Data {
    data.iter()
        .filter(|report| is_unsafe(report))
        .cloned()
        .collect()
}

fn remove_and_check(report: &[i128]) -> bool {
    (0..report.len()).any(|i| {
        let data_without_current: Vec<i128> = report
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, &val)| val)
            .collect();
        
        !is_unsafe(&data_without_current)
    })
}

pub fn maximize_safe_reports(data: Data) -> String {
    let total_reports = data.len();
    let unsafe_reports: Vec<Vec<i128>> = get_unsafe_reports(data);

    #[cfg(debug_assertions)]
    println!("unsafe reports: {:?}", unsafe_reports);

    let mut possible_safe_reports = 0;
    unsafe_reports.iter()
        .for_each(|report| {
            possible_safe_reports += match remove_and_check(report) {
                true => 1,
                false => 0,
            }
        });

    (possible_safe_reports + (total_reports - unsafe_reports.len())).to_string()
}

pub fn solve_part2() -> Result<String> {
    let input = utils::read_input(2024, 02)?;
    let data = parse_input(&input)?;

    let tot = maximize_safe_reports(data);
    Ok(tot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_valid_differences() {
        assert!(has_valid_differences(&[5, 3, 1]));
        assert!(!has_valid_differences(&[1, 5, 6]));
        assert!(has_valid_differences(&[1]));
        assert!(has_valid_differences(&[]));
    }

    #[test]
    fn test_has_trend() {
        assert!(has_trend(&[1, 2, 3], TrendType::Increasing));
        assert!(has_trend(&[5, 3, 1], TrendType::Decreasing));
        assert!(!has_trend(&[1, 3, 2], TrendType::Increasing));
        assert!(has_trend(&[1], TrendType::Increasing));
    }

    #[test]
    fn test_maximize_safe_reports() {
        let data = vec![
            vec![1, 2, 3],
            vec![1, 5, 2],
            vec![1, 5, 8],
        ];
        assert_eq!(maximize_safe_reports(data), "3");
    }
}