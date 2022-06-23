use {
    crate::mesh::Mesh,
    nom::{branch::alt, combinator::map, error::ParseError, IResult},
};

#[cfg(test)]
mod test;

pub(crate) mod v1;
pub(crate) mod v2;
pub(crate) mod v4;

pub fn mesh<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Mesh, E> {
    alt((v1::mesh, v2::mesh, map(v4::mesh, v4::Mesh::to_legacy)))(i)
}
