use super::error::AocError;

pub type AocResult<T> = Result<T, AocError>;
pub type Grid<T> = Vec<Vec<T>>;
pub type Point = (i32, i32);
pub type Coordinate3D = (i32, i32, i32);