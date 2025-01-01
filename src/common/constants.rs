pub const DIAGONAL_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // Up
    (1, 0),  // Down
    (0, -1), // Left
    (0, 1),  // Right
];
