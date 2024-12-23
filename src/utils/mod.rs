use std::fs;
use std::path::Path;

pub fn read_input(year: u32, day: u32) -> Result<String, std::io::Error> {
    let input_path = format!("inputs/{}/day{:02}.txt", year, day);
    fs::read_to_string(Path::new(&input_path))
}
