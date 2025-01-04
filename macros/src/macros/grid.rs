#[macro_export]
macro_rules! char_grid {
    ($input:expr) => {{
        $input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! adjacent {
    ($x:expr, $y:expr, $max_x:expr, $max_y:expr) => {{
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        DIRS.iter()
            .map(|(dx, dy)| ($x as i32 + dx, $y as i32 + dy))
            .filter(|(nx, ny)| {
                *nx >= 0 && *ny >= 0 && *nx < $max_x as i32 && *ny < $max_y as i32
            })
            .map(|(nx, ny)| (nx as usize, ny as usize))
            .collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! adjacent_diagonal {
    ($x:expr, $y:expr, $max_x:expr, $max_y:expr) => {{
        const DIRS: [(i32, i32); 8] = [
            (0, 1), (1, 1), (1, 0), (1, -1),
            (0, -1), (-1, -1), (-1, 0), (-1, 1)
        ];
        DIRS.iter()
            .map(|(dx, dy)| ($x as i32 + dx, $y as i32 + dy))
            .filter(|(nx, ny)| {
                *nx >= 0 && *ny >= 0 && *nx < $max_x as i32 && *ny < $max_y as i32
            })
            .map(|(nx, ny)| (nx as usize, ny as usize))
            .collect::<Vec<_>>()
    }};
}
