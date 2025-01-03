use std::fmt;

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

    pub fn with_dimensions(rows: i32, cols: i32, fill: char) -> Self {
        let cells = vec![vec![fill; cols as usize]; rows as usize];
        Self { rows, cols, cells }
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_width = self.cols as usize * 2; // Each cell takes 2 spaces (char + space)
        let margin = 2; // Left and right padding inside box
        let total_width = content_width + margin;
        let header = format!("[ Grid {}x{} ]", self.rows, self.cols);
        let header_padding = total_width.saturating_sub(header.len()) / 2;
        let border = "═".repeat(total_width);

        // Top border and header
        writeln!(f, "╔{}╗", border)?;
        writeln!(
            f,
            "║{}{}{:>width$}║",
            " ".repeat(header_padding),
            header,
            "",
            width = total_width.saturating_sub(header_padding + header.len())
        )?;
        writeln!(f, "╠{}╣", border)?;

        // Grid content
        for row in &self.cells {
            write!(f, "║")?;
            for &cell in row {
                write!(f, " {}", cell)?;
            }
            writeln!(f, "  ║")?;
        }

        // Bottom border
        writeln!(f, "╚{}╝", border)
    }
}
