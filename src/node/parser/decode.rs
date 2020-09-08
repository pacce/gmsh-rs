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
        number::complete::{
            float,
            double,
        },
        sequence::tuple
    },
    std::{
        collections::HashMap,
        convert::Into,
    },
    super::{
        MAX_INTEGER_TAGS,
        MAX_REAL_TAGS,
        MAX_STRING_TAGS,
        Coordinate,
        Data,
        DataTag,
        Dimension,
        Entity,
        IntegerTag,
        IntegerTags,
        Node,
        Nodes,
        RealTag,
        RealTags,
        StringTag,
        StringTags,
        Tag,
        Value,
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

pub fn nodes<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Nodes, E>
{
    let (i, _)      = complete::tag("$Nodes")(i)?;
    let (i, _)      = newline(i)?;

    let (i, es)     = count(i)?;
    let (i, _)      = space0(i)?;
    let (i, ns)     = count(i)?;
    let (i, _)      = space0(i)?;
    let (i, min)    = tag(i)?;
    let (i, _)      = space0(i)?;
    let (i, max)    = tag(i)?;
    let (i, _)      = newline(i)?;

    let (i, entities) = multi::count(entity, es)(i)?;

    let (i, _)      = complete::tag("$EndNodes")(i)?;
    let (i, _)      = newline(i)?;

    Ok((i, Nodes::new(min, max, entities)))
}

pub fn tag_newline<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Tag, E>
{
    let (i, (t, _)) = tuple((tag, newline))(i)?;
    Ok((i, t))
}

pub fn tag<'a, E: ParseError<&'a str>>(i: &'a str)
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

pub fn node_data<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Data, E>
{
    let (i, _)              = complete::tag("$NodeData")(i)?;
    let (i, _)              = newline(i)?;

    let (i, string_tags)    = string_tags(i)?;
    let (i, real_tags)      = real_tags(i)?;
    let (i, integer_tags)   = integer_tags(i)?;

    let number_of_entities : Option<IntegerTag> = integer_tags.clone().number_of_entities.into();
    let (i, vs) = multi::count(
        value_newline,
        number_of_entities.unwrap() as usize
        )(i)?;

    let mut values : HashMap<Tag, Value> = HashMap::new();
    for (t, v) in vs.iter() {
        values.insert(*t, *v);
    }

    let (i, _)              = complete::tag("$EndNodeData")(i)?;
    let (i, _)              = newline(i)?;

    Ok((i, Data::new(string_tags, real_tags, integer_tags, values)))
}

pub fn string_tags<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, StringTags, E>
{
    let (i, n) = count(i)?;
    let (i, _) = newline(i)?;

    let (i, mut sts) = multi::count(string_tag_newline, n)(i)?;
    sts.resize(MAX_STRING_TAGS, StringTag::default());

    let string_tags = StringTags::new(
        DataTag::Text(StringTag::from(sts[0].clone())),
        DataTag::Text(StringTag::from(sts[1].clone())),
        );

    Ok((i, string_tags))
}

pub fn string_tag_newline<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, StringTag, E>
{
    let (i, (t, _)) = tuple((string_tag, newline))(i)?;
    Ok((i, StringTag::from(t)))
}

pub fn string_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, StringTag, E>
{
    let (i, _) = complete::tag(r#"""#)(i)?;
    let (i, t) = complete::is_not(r#"""#)(i)?;
    let (i, _) = complete::tag(r#"""#)(i)?;
    Ok((i, StringTag::from(t)))
}

pub fn real_tags<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, RealTags, E>
{
    let (i, n) = count(i)?;
    let (i, _) = newline(i)?;

    let (i, mut rts) = multi::count(real_tag_newline, n)(i)?;
    rts.resize(MAX_REAL_TAGS, RealTag::default());

    let real_tags = RealTags::new(DataTag::Real(rts[0]));

    Ok((i, real_tags))
}

pub fn real_tag_newline<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, RealTag, E>
{
    let (i, (t, _)) = tuple((real_tag, newline))(i)?;
    Ok((i, t as RealTag))
}

pub fn real_tag<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, RealTag, E>
{
    let (i, t) = double(i)?;
    Ok((i, t as RealTag))
}

pub fn integer_tags<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, IntegerTags, E>
{
    let (i, n) = count(i)?;
    let (i, _) = newline(i)?;

    let (i, mut its) = multi::count(integer_tag_newline, n)(i)?;
    its.resize(MAX_INTEGER_TAGS, IntegerTag::default());

    let integer_tags = IntegerTags::new(
        DataTag::Integer(its[0]),
        DataTag::Integer(its[1]),
        DataTag::Integer(its[2]),
        DataTag::Integer(its[3])
        );

    Ok((i, integer_tags))
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

pub fn value_newline<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (Tag, Value), E>
{
    let (i, (t, v)) = value(i)?;
    let (i, _) = newline(i)?;
    Ok((i, (t, v as Value)))
}

pub fn value<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (Tag, Value), E>
{
    let (i, t) = tag(i)?;
    let (i, _) = space0(i)?;
    let (i, v) = double(i)?;
    Ok((i, (t, v as Value)))
}
