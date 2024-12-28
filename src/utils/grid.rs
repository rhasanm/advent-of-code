#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub rows: i32,
    pub cols: i32,
}

impl Grid {
    pub fn new(arrangement: &[String]) -> Self {
        let cells: Vec<Vec<char>> = arrangement
            .iter()
            .map(|row| row.chars().collect())
            .collect();
        
        Self {
            rows: cells.len() as i32,
            cols: cells[0].len() as i32,
            cells,
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.rows && y < self.cols
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if self.in_bounds(x, y) {
            Some(self.cells[x as usize][y as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: char) {
        if self.in_bounds(x, y) {
            self.cells[x as usize][y as usize] = value;
        }
    }
}