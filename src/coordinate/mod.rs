#[cfg(test)]
mod test;

use nom::{
    character::complete::space0,
    error::ParseError,
    IResult,
    number::complete::double,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{x, y, z}
    }
}

impl std::default::Default for Coordinate {
    fn default() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0}
    }
}

pub fn decode<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Coordinate, E>
{
    let (i, x) = double(i)?;
    let (i, _) = space0(i)?;
    let (i, y) = double(i)?;
    let (i, _) = space0(i)?;
    let (i, z) = double(i)?;

    Ok((i, Coordinate{x, y, z}))
}
