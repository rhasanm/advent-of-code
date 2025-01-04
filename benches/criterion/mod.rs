use criterion::criterion_main;

pub mod common;
pub mod year2024;

criterion_main! {
    year2024::day07::benches,
}