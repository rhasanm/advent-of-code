use std::collections::HashSet;

use advent_of_code::{solutions::year2024::day05, utils::Graph};
use anyhow::{Ok, Result};
use itertools::Itertools;

const EXAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn test_graph() -> Result<()> {
    let mut graph: Graph<i32> = Graph::new();

    graph.add_edge(1, 2);
    graph.add_edge(1, 3);

    assert_eq!(graph.neighbors(&1), Some(&HashSet::from([2, 3])));

    let data = day05::parse_input(EXAMPLE_INPUT)?;
    let page_ordering = day05::parse_page_ordering(&data);

    let mut graph: Graph<i32> = Graph::new();

    page_ordering
        .iter()
        .for_each(|edge| graph.add_edge(edge.0, edge.1));

    println!("{:?}", graph);
    Ok(())
}

#[test]
fn test_parse_page_ordering() -> Result<()> {
    let data = day05::parse_input(EXAMPLE_INPUT)?;
    let page_ordering = day05::parse_page_ordering(&data);

    assert_eq!(page_ordering[0], (47, 53));
    Ok(())
}

#[test]
fn test_parse_page_update() -> Result<()> {
    let data = day05::parse_input(EXAMPLE_INPUT)?;
    let page_updates = day05::parse_page_update(&data);

    assert_eq!(page_updates[5], vec![75, 47, 61, 53, 29]);
    Ok(())
}

#[test]
fn test_part1_example() -> Result<()> {
    let data = day05::parse_input(EXAMPLE_INPUT)?;
    let page_ordering = day05::parse_page_ordering(&data);
    let page_updates = day05::parse_page_update(&data);

    let mut graph: Graph<i32> = Graph::new();

    page_ordering
        .iter()
        .for_each(|edge| graph.add_edge(edge.0, edge.1));

    let correctly_ordered = page_updates
        .iter()
        .filter_map(|updates| {
            if updates.iter().enumerate().all(|(idx, page)| {
                updates
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| index > &idx)
                    .all(|(_, node)| !graph.neighbors(node).unwrap().contains(page))
            }) {
                Some(updates)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    let sum = correctly_ordered
        .iter()
        .map(|updates| updates.get(updates.len() / 2).unwrap())
        .sum::<i32>();

    assert_eq!(sum, 143);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let solution = day05::solve_part1()?;
    println!("Solution Part 1: {}", solution);

    assert_eq!(solution, 6949);
    Ok(())
}

#[test]
fn test_part2_example() -> Result<()> {
    let data = day05::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let solution = day05::solve_part2()?;
    println!("Solution Part 2: {}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}
