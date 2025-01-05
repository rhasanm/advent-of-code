use criterion::Criterion;

pub fn create_benchmark_group<'a>(
    c: &'a mut Criterion,
    name: &'a str,
) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut group = c.benchmark_group(name);
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));
    group
}

pub fn load_test_input(year: u16, day: u8) -> String {
    std::fs::read_to_string(format!("inputs/{}/day{:02}.txt", year, day))
        .expect("Failed to read input file")
}
