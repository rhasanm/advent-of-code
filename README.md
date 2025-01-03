# Advent of Code Solutions in Rust 🦀

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) challenges implemented in Rust. The project is structured to support solutions from multiple years in a maintainable way.

## 🚀 Project Structure

```
.
├── src/
│   ├── solutions/      # Solutions organized by year
│   │   ├── year2022/
│   │   ├── year2023/
│   │   └── year2024/
│   ├── benchmark/      # Benchmark infrastructure
│   ├── common/        # Shared traits and core functionality
│   ├── utils/         # Shared utility functions
│   └── bin/           # Command-line tools
├── inputs/            # Input files for each day
│   ├── 2022/
│   ├── 2023/
│   └── 2024/
├── outputs/           # Generated output files (e.g., images, logs)
│   ├── 2022/
│   ├── 2023/
│   └── 2024/
└── tests/             # Test files matching the solution structure
    ├── 2022/
    ├── 2023/
    └── 2024/
```

## 🛠️ Setup

1. Clone the repository:
```bash
git clone https://github.com/rhasanm/advent-of-code.git
cd advent-of-code
```

2. Build the project:
```bash
cargo build
```

## 🎯 Running Solutions

Run a specific solution using:
```bash
cargo run <year> <day> <part>
```

Examples:
```bash
# Run Year 2023, Day 1, Part 1
cargo run 2023 1 1

# Run Year 2023, Day 1, Part 2
cargo run 2023 1 2
```

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Run tests for a specific day:
```bash
# Test Year 2023, Day 1
cargo test year2023::day01

# Test specific part of Day 1
cargo test year2023::day01::part1
```

## 🚀 Benchmarking

Run benchmarks for specific solutions:
```bash
cargo run --bin bench -y <year> -d <day>
```

Example:
```bash
# Benchmark Year 2024, Day 7 solutions
cargo run --bin bench -y 2024 -d 7
```

This will run performance benchmarks using Criterion.rs and output detailed metrics for different solution approaches.

## 📝 Adding New Solutions

You can use our custom command-line tool to create new solution files:

```bash
# Create solution files for a specific year and day
cargo run --bin aoc new -y 2024 -d 1
```

This will automatically:
1. Create the solution file with template code
2. Create the test file with example and solution tests
3. Create an empty input file
4. Update all necessary mod.rs files

Or manually create files:

1. Create a new solution file:
```bash
touch src/solutions/year2024/day01.rs
```

2. Add the module to `src/solutions/year2024/mod.rs`:
```rust
pub mod day01;
```

3. Create the corresponding test file:
```bash
touch tests/solutions/year2024/day01_test.rs
```

4. Add your input file:
```bash
touch inputs/2024/day01.txt
```

## 🔧 Development Tools

- Run formatter:
```bash
cargo fmt
```

- Run linter:
```bash
cargo clippy
```

## 📚 Dependencies

- `regex`: For string parsing and matching
- `lazy_static`: For lazy evaluation of static variables
- `itertools`: For additional iterator operations
- `rayon`: For parallel processing
- `anyhow`: For error handling
- `clap`: For command-line argument parsing
- `criterion`: For performance benchmarking

## 💡 Tips

- Use the `aoc` command-line tool to generate new solution files
- Input files should be named as `dayXX.txt` (e.g., `day01.txt`, `day02.txt`)
- Each day's solution should implement both `solve_part1()` and `solve_part2()`
- Add tests using the example inputs provided in the problem description
- Use the common traits in `common/traits.rs` for numeric operations
- Use the utility functions in `utils/mod.rs` for common operations

## 🤝 Contributing

1. Create a new branch for your solution
2. Implement your solution and tests
3. Run tests and linting
4. Create a pull request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.
