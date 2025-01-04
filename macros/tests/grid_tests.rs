use aoc_macros::adjacent;

#[test]
fn test_adjacent_grid_navigation() {
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    let height = grid.len();
    let width = grid[0].len();

    let neighbors = adjacent!(1, 1, width, height);
    
    let adjacent_values: Vec<i32> = neighbors
        .iter()
        .map(|&(x, y)| grid[y][x])
        .collect();

    assert_eq!(adjacent_values, vec![8, 6, 2, 4]);
}