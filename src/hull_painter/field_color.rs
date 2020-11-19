
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug,PartialEq,Clone)]
pub enum FieldColor {
    White, Black
}

impl TryFrom<i64> for FieldColor {
    type Error = FieldColorError;
    fn try_from(i: i64) -> std::result::Result<Self, <Self as std::convert::TryFrom<i64>>::Error> {
        match i {
            0 => Ok(FieldColor::Black),
            1 => Ok(FieldColor::White),
            _ => Err(FieldColorError{source: i})
        } 
    }
}

impl FieldColor {
    pub fn to_intputer_input(&self) -> i64 {
        match self {
            FieldColor::Black => 0,
            FieldColor::White => 1,
        }
    }
}
#[derive(Debug)]
pub struct FieldColorError {source: i64}

impl Error for FieldColorError{}

impl Display for FieldColorError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        fmt.write_fmt(format_args!("unable to map number {}", self.source))
    }
}