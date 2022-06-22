use {
    crate::mesh::Mesh,
    nom::{branch::alt, error::ParseError, IResult},
};

#[cfg(test)]
mod test;

pub(crate) mod v1;
pub(crate) mod v2;
pub(crate) mod v4;

pub fn mesh<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Mesh, E> {
    alt((v1::mesh, v2::mesh, v4::mesh))(i)
}
