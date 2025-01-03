use num_traits::{PrimInt, Signed};
use std::{
    ops::{Add, Mul},
    str::FromStr,
};

pub trait Number:
    Copy + Add<Output = Self> + Mul<Output = Self> + FromStr + PartialEq + Default + PrimInt + Signed
{
}

impl<T> Number for T where
    T: Copy + Add<Output = T> + Mul<Output = T> + FromStr + PartialEq + Default + PrimInt + Signed
{
}
