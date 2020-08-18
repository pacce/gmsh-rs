use {
    nom::{
        character::complete::{
            newline,
            space0,
        },
        error::ParseError,
        IResult,
        multi,
        number::complete::{
            float,
            double,
        },
        sequence::tuple
    },
    std::collections::HashMap,
    super::{
        Coordinate,
        Dimension,
        Entity,
        Node,
        Tag,
    },
};

pub fn entity<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Entity, E>
{
    let (i, dim)            = dimension(i)?;
    let (i, _)              = space0(i)?;
    let (i, tag)            = tag(i)?;
    let (i, _)              = space0(i)?;
    let (i, _parametric)    = parametric(i)?;
    let (i, _)              = space0(i)?;
    let (i, count)          = count(i)?;
    let (i, _)              = newline(i)?;

    let mut nodes : HashMap<Tag, Node> = HashMap::new();

    let (i, ts) = multi::count(tag_newline, count)(i)?;
    let (i, ns) = multi::count(node, count)(i)?;

    for (t, n) in ts.iter().zip(ns) {
        nodes.insert(*t, n);
    }

    Ok((i, Entity::new(dim, tag, nodes)))
}

pub fn node<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Node, E>
{
    let (i, (n, _)) = tuple((coordinates, newline))(i)?;
    Ok((i, n))
}

pub fn tag_newline<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Tag, E>
{
    let (i, (t, _)) = tuple((tag, newline))(i)?;
    Ok((i, t))
}

fn tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Tag, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as Tag))
}

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

fn count<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, usize, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as usize))
}

fn dimension<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Dimension, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as Dimension))
}

fn parametric<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, bool, E>
{
    let (i, n) = float(i)?;
    Ok((i, n == 1.0))
}
