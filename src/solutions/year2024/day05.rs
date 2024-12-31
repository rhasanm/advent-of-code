use crate::utils::prelude::{read_input, Graph};
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn parse_page_ordering(input: &Vec<String>) -> Vec<(i32, i32)> {
    input
        .iter()
        .take_while(|line| line.ne(&""))
        .map(|line| {
            let order = line.split("|").collect::<Vec<&str>>();
            (order[0].parse().unwrap(), order[1].parse().unwrap())
        })
        .collect()
}

pub fn parse_page_update(input: &Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .rev()
        .take_while(|line| line.ne(&""))
        .map(|line| {
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|str| str.parse().unwrap())
                .collect()
        })
        .collect()
}

fn build_graph(page_ordering: &[(i32, i32)]) -> Graph<i32> {
    let mut graph = Graph::new();

    page_ordering
        .iter()
        .for_each(|&(from, to)| graph.add_edge(from, to));

    graph
}

pub fn solve_part1() -> Result<i32> {
    let input = read_input(2024, 5)?;
    let data = parse_input(&input)?;

    let page_ordering = parse_page_ordering(&data);
    let page_updates = parse_page_update(&data);

    let graph = build_graph(&page_ordering);

    let correctly_ordered = page_updates
        .iter()
        .filter_map(|updates| {
            if updates.iter().enumerate().all(|(idx, page)| {
                updates
                    .iter()
                    .enumerate()
                    .skip(idx + 1)
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

    Ok(sum)
}

pub fn solve_part2() -> Result<String> {
    let input = read_input(2024, 5)?;
    let data = parse_input(&input)?;

    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}
