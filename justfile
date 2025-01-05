# Development Setup
setup:
    cp .env.example .env
    pre-commit install
    cargo install cargo-fuzz
    cargo install cargo-criterion
    git config --local commit.template .gitmessage

# Code Quality
format:
    cargo fmt
    
lint:
    cargo clippy -- -D warnings

lint-dependencies:
    cargo deny check

check: format lint
    cargo check
    pre-commit run --all-files

# Solution Management
new-year-day year day:
    cargo run --bin aoc new -y {{year}} -d {{day}}

# Running Solutions
run year day part:
    cargo run --bin aoc {{year}} {{day}} {{part}}

run-release year day part:
    cargo run --release --bin aoc {{year}} {{day}} {{part}}

# Testing
test:
    cargo test

test-day year day:
    cargo test year{{year}}::day{{day}}

# Benchmarking
bench-custom year day:
    cargo run --bin bench -- -y {{year}} -d {{day}}

bench-criterion:
    cargo bench --bench solutions_2024

bench-iai:
    cargo bench --bench iai_solutions

bench-all: bench-criterion bench-iai

# Fuzzing
fuzz-new target:
    cargo fuzz add {{target}}

fuzz-run target:
    cargo fuzz run {{target}}

# Docker
docker-build:
    docker-compose build

docker-run year day part:
    docker-compose run app cargo run --bin aoc {{year}} {{day}} {{part}}

# Cross Compilation
cross-build target:
    cross build --target {{target}}

cross-build-release target:
    cross build --release --target {{target}}

# Git Workflow
commit-template:
    git config --local commit.template .gitmessage

# Clean
clean:
    cargo clean
    rm -rf target/
    rm -rf fuzz/target/

# Full Development Cycle
dev-cycle: 
    check test bench-criterion

# Documentation
doc:
    cargo doc --no-deps --open

# Watch Mode (for development)
watch:
    cargo watch -x check -x test

# Default
default:
    @just --list