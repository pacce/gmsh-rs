use {
    super::{
        Coordinate,
        Node,
        Tag,
    },
    nom::{
        character::complete::{
            newline,
            space0,
        },
        error::ParseError,
        IResult,
        number::complete::{
            float,
            double,
        },
        sequence::tuple
    }
};

fn coordinate<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Coordinate, E>
{
    double(i)
}

pub(super) fn coordinates<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Node, E>
{
    let (i, x) = coordinate(i)?;
    let (i, _) = space0(i)?;
    let (i, y) = coordinate(i)?;
    let (i, _) = space0(i)?;
    let (i, z) = coordinate(i)?;

    Ok((i, Node::new(x, y, z)))
}

pub fn node<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Node, E>
{
    let (i, (n, _)) = tuple((coordinates, newline))(i)?;
    Ok((i, n))
}

fn tagv<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Tag, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as Tag))
}

pub fn tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Tag, E>
{
    let (i, (t, _)) = tuple((tagv, newline))(i)?;
    Ok((i, t))
}
