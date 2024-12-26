use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::io::Write;

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code solution generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(short, long)]
        year: u32,
        #[arg(short, long)]
        day: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { year, day } => {
            create_solution_files(year, day).unwrap_or_else(|err| {
                eprintln!("Error creating files: {}", err);
                std::process::exit(1);
            });
        }
    }
}

fn create_solution_files(year: u32, day: u32) -> std::io::Result<()> {
    // Create directories if they don't exist
    let solution_dir = format!("src/solutions/year{}", year);
    let test_dir = format!("tests/solutions/year{}", year);
    let input_dir = format!("inputs/{}", year);

    fs::create_dir_all(&solution_dir)?;
    fs::create_dir_all(&test_dir)?;
    fs::create_dir_all(&input_dir)?;

    // Create/update mod.rs in solution directory
    let mod_path = format!("{}/mod.rs", solution_dir);
    let day_module = format!("day{:02}", day);
    if !Path::new(&mod_path).exists() {
        fs::write(&mod_path, format!("pub mod {};", day_module))?;
    } else {
        let mut content = fs::read_to_string(&mod_path)?;
        if !content.contains(&day_module) {
            content.push_str(&format!("\npub mod {};", day_module));
            fs::write(&mod_path, content)?;
        }
    }

    // Create solution file
    let solution_file = format!("{}/day{:02}.rs", solution_dir, day);
    if !Path::new(&solution_file).exists() {
        let mut file = fs::File::create(&solution_file)?;
        write!(file, r#"use crate::utils;
use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<String>> {{
    Ok(input.lines().map(String::from).collect())
}}

pub fn solve_part1() -> Result<String> {{
    let input = utils::read_input({}, {})?;
    let data = parse_input(&input)?;
    
    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}}

pub fn solve_part2() -> Result<String> {{
    let input = utils::read_input({}, {})?;
    let data = parse_input(&input)?;
    
    // TODO: Implement solution
    Ok("Not implemented yet".to_string())
}}"#, year, day, year, day)?;
    }

    // Create test file
    let test_file = format!("{}/day{:02}_test.rs", test_dir, day);
    if !Path::new(&test_file).exists() {
        let mut file = fs::File::create(&test_file)?;
        write!(file, r#"use advent_of_code::solutions::year{}::day{:02};
use anyhow::Result;

const EXAMPLE_INPUT: &str = "\
first line
second line
third line";

#[test]
fn test_part1_example() -> Result<()> {{
    let data = day{:02}::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}}

#[test]
fn test_part1_solution() -> Result<()> {{
    let solution = day{:02}::solve_part1()?;
    println!("Solution Part 1: {{}}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}}

#[test]
fn test_part2_example() -> Result<()> {{
    let data = day{:02}::parse_input(EXAMPLE_INPUT)?;
    // TODO: Add test implementation
    Ok(())
}}

#[test]
fn test_part2_solution() -> Result<()> {{
    let solution = day{:02}::solve_part2()?;
    println!("Solution Part 2: {{}}", solution);
    // TODO: Once you have the correct answer, uncomment and update:
    // assert_eq!(solution, "expected_answer");
    Ok(())
}}"#, year, day, day, day, day, day)?;
    }

    // Create/update tests/solutions/yearXXXX/mod.rs
    let tests_year_mod_path = format!("tests/solutions/year{}/mod.rs", year);
    if !Path::new(&tests_year_mod_path).exists() {
        fs::create_dir_all(format!("tests/solutions/year{}", year))?;
        fs::write(&tests_year_mod_path, &format!("mod day{:02}_test;", day))?;
    } else {
        let mut content = fs::read_to_string(&tests_year_mod_path)?;
        if !content.contains(&format!("mod day{:02}_test;", day)) {
            content.push_str(&format!("\nmod day{:02}_test;", day));
            fs::write(&tests_year_mod_path, content)?;
        }
    }

    // Create empty input file
    let input_file = format!("{}/day{:02}.txt", input_dir, day);
    if !Path::new(&input_file).exists() {
        fs::File::create(&input_file)?;
    }

    println!("Created files for Year {}, Day {}:", year, day);
    println!("  - {}/day{:02}.rs", solution_dir, day);
    println!("  - {}/day{:02}_test.rs", test_dir, day);
    println!("  - {}/day{:02}.txt", input_dir, day);

    Ok(())
}