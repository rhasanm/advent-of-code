# Advent of Code Solutions in Rust 🦀

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) challenges implemented in Rust. The project is structured to support solutions from multiple years in a maintainable way.

## 🚀 Project Structure

```
.
├── src/
│   ├── solutions/       # Solutions organized by year
│   │   ├── year2022/
│   │   └── year2023/
│   └── utils/          # Shared utility functions
├── inputs/             # Input files organized by year
│   ├── 2022/
│   └── 2023/
└── tests/             # Test files matching the solution structure
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

## 📝 Adding New Solutions

1. Create a new solution file:
```bash
# For Year 2023, Day 3
touch src/solutions/year2023/day03.rs
```

2. Add the module to `src/solutions/year2023/mod.rs`:
```rust
pub mod day03;
```

3. Create the corresponding test file:
```bash
touch tests/solutions/year2023/day03_test.rs
```

4. Add your input file:
```bash
touch inputs/2023/day03.txt
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

## 💡 Tips

- Input files should be named as `dayXX.txt` (e.g., `day01.txt`, `day02.txt`)
- Each day's solution should implement both `solve_part1()` and `solve_part2()`
- Add tests using the example inputs provided in the problem description
- Use the utility functions in `utils/mod.rs` for common operations

## 🤝 Contributing

1. Create a new branch for your solution
2. Implement your solution and tests
3. Run tests and linting
4. Create a pull request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.