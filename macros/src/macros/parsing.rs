#[macro_export]
macro_rules! parse_lines {
    ($input:expr, $type:ty) => {
        $input.lines().map(|line| line.parse::<$type>().unwrap())
    };
}

#[macro_export]
macro_rules! parse_numbers {
    ($input:expr, $type:ty) => {
        $input
            .split_whitespace()
            .map(|n| n.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
    };
}