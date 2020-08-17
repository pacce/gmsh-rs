use {
    crate::node,
    nom::{
        character::complete::{
            newline,
            space0,
        },
        error::ParseError,
        IResult,
        multi::count,
        number::complete::float,
    },
    std::collections::HashMap,
    super::{
        Dimension,
        Entity,
        Tag,
    },
};

fn dimension<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Dimension, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as Dimension))
}

fn tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Tag, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as Tag))
}

fn parametric<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, bool, E>
{
    let (i, n) = float(i)?;
    Ok((i, n == 1.0))
}

fn node_count<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, usize, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as usize))
}

pub fn entity<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Entity, E>
{
    let (i, dimension)      = dimension(i)?;
    let (i, _)              = space0(i)?;
    let (i, tag)            = tag(i)?;
    let (i, _)              = space0(i)?;
    let (i, _parametric)    = parametric(i)?;
    let (i, _)              = space0(i)?;
    let (i, node_count)     = node_count(i)?;
    let (i, _)              = newline(i)?;

    let (i, nodes) = {
        let (i, ts) = count(node::decode::tag, node_count)(i)?;
        let (i, ns) = count(node::decode::node, node_count)(i)?;

        let mut hs : HashMap<node::Tag, node::Node> = HashMap::new();
        for (t, n) in ts.iter().zip(ns) {
            hs.insert(*t, n);
        }
        (i, hs)
    };

    let e = Entity{
        dimension,
        tag,
        nodes,
    };
    Ok((i, e))
}
