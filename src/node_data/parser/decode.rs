use {
    crate::node,
    nom::{
        bytes::complete,
        character::complete::{
            newline,
            space0,
        },
        combinator::rest,
        error::ParseError,
        IResult,
        multi,
        number::complete::{
            float,
            double
        },
        sequence::tuple,
    },
    std::collections::HashMap,
    super::{
        IntegerTag,
        RealTag,
        StringTag,
        Value,
    }
};

pub fn string_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, StringTag, E>
{
    let (i, _) = complete::tag(r#"""#)(i)?;
    let (i, t) = complete::is_not(r#"""#)(i)?;
    Ok((i, StringTag::from(t)))
}

pub fn real_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, RealTag, E>
{
    let (i, t) = double(i)?;
    Ok((i, t as RealTag))
}

pub fn integer_tag_newline<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, IntegerTag, E>
{
    let (i, (t, _)) = tuple((integer_tag, newline))(i)?;
    Ok((i, t as IntegerTag))
}

pub fn integer_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, IntegerTag, E>
{
    let (i, t) = float(i)?;
    Ok((i, t as IntegerTag))
}

/*
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
        3 => multi::count(quadrangle4, count)(i)?,
        4 => multi::count(tetrahedron4, count)(i)?,
        5 => multi::count(hexahedron8, count)(i)?,
        6 => multi::count(prism6, count)(i)?,
        7 => multi::count(pyramid5, count)(i)?,
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

pub fn quadrangle4<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n2) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n3) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Quadrangle4 (
            n0 as node::Tag,
            n1 as node::Tag,
            n2 as node::Tag,
            n3 as node::Tag,
        )
    )))
}

pub fn tetrahedron4<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n2) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n3) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Tetrahedron4 (
            n0 as node::Tag,
            n1 as node::Tag,
            n2 as node::Tag,
            n3 as node::Tag,
        )
    )))
}

pub fn hexahedron8<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n2) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n3) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n4) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n5) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n6) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n7) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Hexahedron8 (
            n0 as node::Tag,
            n1 as node::Tag,
            n2 as node::Tag,
            n3 as node::Tag,
            n4 as node::Tag,
            n5 as node::Tag,
            n6 as node::Tag,
            n7 as node::Tag,
        )
    )))
}

pub fn prism6<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n2) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n3) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n4) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n5) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Prism6 (
            n0 as node::Tag,
            n1 as node::Tag,
            n2 as node::Tag,
            n3 as node::Tag,
            n4 as node::Tag,
            n5 as node::Tag,
        )
    )))
}

pub fn pyramid5<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (ElementTag, Element), E>
{
    let (i, t)  = element_tag(i)?;
    let (i, _)  = space0(i)?;
    let (i, n0) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n1) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n2) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n3) = float(i)?;
    let (i, _)  = space0(i)?;
    let (i, n4) = float(i)?;
    let (i, _)  = newline(i)?;

    Ok((i, (
        t,
        Element::Pyramid5 (
            n0 as node::Tag,
            n1 as node::Tag,
            n2 as node::Tag,
            n3 as node::Tag,
            n4 as node::Tag,
        )
    )))
}
*/
