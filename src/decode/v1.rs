use {
    nom::{
        bytes::complete,
        character::complete::{
            newline,
            space0,
        },
        error::ParseError,
        IResult,
        multi,
        number::complete::{double},
    },
    crate::node::{Coordinate, Id, Node},
};

pub fn coordinate<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Coordinate, E>
{
    double(i)
}

pub fn id<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Id, E>
{
    let (i, n) = double(i)?;
    Ok((i, n as Id))
}

pub fn node<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Node, E>
{
    let (i, id) = id(i)?;
    let (i, _)  = space0(i)?;

    let (i, x)  = coordinate(i)?;
    let (i, _)  = space0(i)?;
    let (i, y)  = coordinate(i)?;
    let (i, _)  = space0(i)?;
    let (i, z)  = coordinate(i)?;

    let (i, _)  = newline(i)?;
    Ok((i, Node::new(id, x, y, z)))
}

pub fn nodes<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Vec<Node>, E>
{
    let (i, _)  = complete::tag("$NOD")(i)?;
    let (i, _)  = newline(i)?;

    let (i, n)  = double(i)?;
    let (i, _)  = newline(i)?;

    let (i, ns) = multi::count(node, n as usize)(i)?;

    let (i, _)  = complete::tag("$ENDNOD")(i)?;
    Ok((i, ns))
}
