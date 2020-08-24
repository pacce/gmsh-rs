use {
    crate::node,
    nom::{
        bytes::complete,
        character::complete::{
            newline,
            space0,
        },
        error::ParseError,
        IResult,
        multi,
        number::complete::float,
        sequence::tuple,
    },
    std::collections::HashMap,
    super::{
        Dimension,
        Element,
        Elements,
        ElementTag,
        ElementType,
        Entity,
        EntityTag,
    }
};

pub fn elements<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Elements, E>
{
    let (i, _)      = complete::tag("$Elements")(i)?;
    let (i,_)       = newline(i)?;

    let (i, ens)    = count(i)?;
    let (i, _)      = space0(i)?;
    let (i, es)     = count(i)?;
    let (i, _)      = space0(i)?;
    let (i, min)    = element_tag(i)?;
    let (i, _)      = space0(i)?;
    let (i, max)    = element_tag(i)?;
    let (i, _)      = newline(i)?;

    let (i, entities) = multi::count(entity, ens)(i)?;

    let (i, _)      = complete::tag("$EndElements")(i)?;
    let (i, _)      = newline(i)?;

    Ok((i, Elements::new(min, max, entities)))
}

pub fn entity<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Entity, E>
{
    let (i, dim)            = dimension(i)?;
    let (i, _)              = space0(i)?;
    let (i, entity_tag)     = entity_tag(i)?;
    let (i, _)              = space0(i)?;
    let (i, element_type)   = element_type(i)?;
    let (i, _)              = space0(i)?;
    let (i, count)          = count(i)?;
    let (i, _)              = newline(i)?;

    let mut elements : HashMap<ElementTag, Element> = HashMap::new();

    let (i, es) = match element_type {
        1 => multi::count(line2, count)(i)?,
        2 => multi::count(triangle3, count)(i)?,
        _ => ("", vec![]),
    };

    for (t, e) in es.iter() {
        elements.insert(*t, *e);
    }

    Ok((i, Entity::new(dim, entity_tag, element_type, elements)))
}

pub fn count<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, usize, E>
{
    let (i, t) = float(i)?;
    Ok((i, t as usize))
}

pub fn dimension<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Dimension, E>
{
    let (i, t) = float(i)?;
    Ok((i, t as Dimension))
}

pub fn entity_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, EntityTag, E>
{
    let (i, t) = float(i)?;
    Ok((i, t as EntityTag))
}

pub fn element_type<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, ElementType, E>
{
    let (i, t) = float(i)?;
    Ok((i, t as ElementType))
}

pub fn element_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, ElementTag, E>
{
    let (i, t) = float(i)?;
    Ok((i, t as ElementTag))
}

pub fn line2<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Line2 (
            n0 as node::Tag,
            n1 as node::Tag,
        )
    )))
}

pub fn triangle3<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n2) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Triangle3 (
            n0 as node::Tag,
            n1 as node::Tag,
            n2 as node::Tag,
        )
    )))
}
