commit-template:
   git config --local commit.template .gitmessage

new year day:
   cargo run --bin aoc new -y {{year}} -d {{day}}

bench year day:
   cargo run --bin bench -- -y {{year}} -d {{day}}

run year day part:
   cargo run --bin advent_of_code {{year}} {{day}} {{part}}