# Advent of Code Solutions in Rust 🦀

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) challenges implemented in Rust. The project is structured as a comprehensive Rust workspace with industry-standard practices for testing, benchmarking, and development.

## 🏗️ Architecture Overview

### Project Structure
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
├── benches/              # Performance benchmarks
│   ├── criterion/        # Criterion.rs benchmarks
│   └── iai/             # CPU instruction benchmarks
├── fuzz/                 # Fuzzing infrastructure
│   ├── Cargo.toml       # Fuzzing dependencies
│   └── fuzz_targets/    # Fuzzing test cases
├── inputs/              # Challenge input files
├── outputs/             # Generated outputs
├── tests/               # Test suite
├── tools/               # Custom development tools
├── macros/             # Procedural macros
├── docs/               # Documentation
├── .cargo/             # Cargo configuration
│   └── config.toml     # Cargo settings
├── .env                # Local environment variables
├── .env.example        # Example environment configuration
├── .dockerignore       # Docker ignore patterns
├── .editorconfig       # Editor configuration
├── .pre-commit-config.yaml  # Pre-commit hook configuration
├── .rustfmt.toml      # Rust formatting rules
├── .clippy.toml       # Clippy linter configuration
├── Cross.toml         # Cross-compilation settings
├── Dockerfile         # Container definition
├── docker-compose.yml # Container orchestration
├── lefthook.yml      # Git hooks manager
└── rust-toolchain.toml # Rust version and components
```

### Core Components

#### Solutions (`src/solutions/`)
- Year-based organization (e.g., `year2024/`)
- Each day implements the `Solution` trait
- Consistent interface for all puzzles
- Modular and testable design

#### Utilities (`src/utils/`)
- `grid.rs`: 2D grid operations
- `graph.rs`: Graph algorithms
- `math.rs`: Mathematical utilities
- `parser.rs`: Input parsing helpers
- `visualization.rs`: Output visualization
- `base_conversion.rs`: Number base utilities
- `input.rs`: Input file handling

#### Common (`src/common/`)
- `traits.rs`: Shared traits and interfaces
- `types.rs`: Common type definitions
- `error.rs`: Error handling
- `constants.rs`: Global constants
- `prelude.rs`: Commonly used imports

#### CLI Tools (`src/bin/`)
- `aoc.rs`: Solution runner and file generator
- `bench.rs`: Benchmark runner

## 🚀 Getting Started

### Prerequisites
- Rust (see `rust-toolchain.toml` for version)
- Cargo
- Just (optional, for task automation)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/rhasanm/advent-of-code.git
cd advent-of-code
```

2. Install development tools:
```bash
cargo install cargo-fuzz    # For fuzzing tests
cargo install cargo-criterion # For benchmarking
cargo install just         # For task automation
```

3. Set up pre-commit hooks:
```bash
pre-commit install
```

## 💻 Development Workflow

### Creating New Solutions

Using the CLI tool:
```bash
cargo run --bin aoc new -y 2024 -d 1
```

This generates:
```
src/solutions/year2024/day01.rs   # Solution implementation
tests/solutions/year2024/day01_test.rs  # Tests
inputs/2024/day01.txt             # Input file
```

### Running Solutions

```bash
# Run specific solution
cargo run --bin aoc run -y 2024 -d 1 -p 1

# Run all solutions for a year
cargo run --bin aoc run -y 2024

# Run with release optimizations
cargo run --release --bin aoc run -y 2024 -d 1
```

### Testing Infrastructure

```bash
# Run all tests
cargo test

# Run tests for specific day
cargo test year2024::day01

# Run tests with features
cargo test --features visualization
```

### Benchmarking

Custom benchmarks:
```bash
cargo run --bin bench -y 2024 -d 7
```

Criterion benchmarks:
```bash
cargo bench --bench solutions_2024
```

Instruction analysis:
```bash
cargo bench --bench iai_solutions
```

### Fuzzing

```bash
# Run fuzzer on specific target
cargo fuzz run day07_parser

# List available targets
cargo fuzz list
```

## 🔧 Development Tools

### Code Quality
```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Run all checks
just check
```

### Docker Support
```bash
# Build container
docker build -t aoc-rust .

# Run solutions in container
docker run aoc-rust cargo run --bin aoc run -y 2024 -d 1
```

## 📚 Documentation

- Architecture decisions are documented in `docs/`
- Solutions include inline documentation
- Utility modules are documented with examples
- Benchmark results are in `docs/benchmarks/`

## 🔍 Implementation Guidelines

### Solution Structure
```rust
pub struct Day01;

impl Solution for Day01 {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse(input: &str) -> Self::Input {
        // Parse input
    }

    fn part1(input: &Self::Input) -> Self::Output {
        // Solve part 1
    }

    fn part2(input: &Self::Input) -> Self::Output {
        // Solve part 2
    }
}
```

### Best Practices
- Use type-driven development
- Write tests for example inputs
- Benchmark performance-critical code
- Document complex algorithms
- Use utility modules for common operations
- Follow Rust idioms and patterns

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your solution
4. Add tests and documentation
5. Run all checks: `just check`
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Advent of Code](https://adventofcode.com/) for the challenges
- Rust community for the amazing ecosystem
- Contributors to the utility crates