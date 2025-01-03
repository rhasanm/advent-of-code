use plotters::prelude::*;
use crossterm::{
    cursor, execute,
    style::{Color as CrosstermColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn print_colored_grid(grid: &[Vec<char>]) {
    for row in grid {
        for &cell in row {
            let color = match cell {
                '#' => "\x1b[31m", // Red for corrupted cells
                '.' => "\x1b[32m", // Green for safe cells
                'O' => "\x1b[33m", // Yellow for visited cells
                _ => "\x1b[0m",    // Reset color
            };
            print!("{}{}\x1b[0m ", color, cell);
        }
        println!();
    }
}

pub fn render_grid_as_image(grid: &[Vec<char>], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let cell_size = 10;
    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            let color = match cell {
                '#' => RGBColor(255, 0, 0),    // Red
                '.' => RGBColor(0, 255, 0),    // Green
                'O' => RGBColor(255, 255, 0),  // Yellow
                _ => RGBColor(0, 0, 0),        // Black
            };
            root.draw(&Rectangle::new(
                [
                    ((y * cell_size) as i32, (x * cell_size) as i32),
                    (((y + 1) * cell_size) as i32, ((x + 1) * cell_size) as i32),
                ],
                color.filled(),
            ))?;
        }
    }

    Ok(())
}

pub fn render_grid_interactive(grid: &[Vec<char>]) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            let color = match cell {
                '#' => CrosstermColor::Red,
                '.' => CrosstermColor::Green,
                'O' => CrosstermColor::Yellow,
                _ => CrosstermColor::White,
            };
            execute!(
                stdout,
                cursor::MoveTo(y as u16, x as u16),
                SetForegroundColor(color),
                Print(cell),
                ResetColor
            )?;
        }
    }

    stdout.flush()?;
    Ok(())
}