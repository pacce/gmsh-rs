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
    super::*,
};

pub fn node_data<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, NodeData, E>
{
    let (i, _)              = complete::tag("$NodeData")(i)?;
    let (i, _)              = newline(i)?;

    let (i, string_tags)    = string_tags(i)?;
    let (i, real_tags)      = real_tags(i)?;
    let (i, integer_tags)   = integer_tags(i)?;
    let (i, vs)             = multi::count(
        value_newline,
        integer_tags.number_of_entities as usize
        )(i)?;

    let mut values : HashMap<node::Tag, Value> = HashMap::new();
    for (t, v) in vs.iter() {
        values.insert(*t, *v);
    }

    let (i, _)              = complete::tag("$EndNodeData")(i)?;
    let (i, _)              = newline(i)?;

    Ok((i, NodeData::new(string_tags, real_tags, integer_tags, values)))
}

pub fn count<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, usize, E>
{
    let (i, n) = float(i)?;
    Ok((i, n as usize))
}

pub fn string_tags<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, StringTags, E>
{
    let (i, n) = count(i)?;
    let (i, _) = newline(i)?;

    let (i, mut sts) = multi::count(string_tag_newline, n)(i)?;
    sts.resize(MAX_STRING_TAGS, StringTag::default());

    let string_tags = StringTags::new(
        sts[0].clone(),
        sts[1].clone()
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

    let real_tags = RealTags::new(rts[0]);

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
        its[0],
        its[1],
        its[2],
        its[3]
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
    -> IResult<&'a str, (node::Tag, Value), E>
{
    let (i, (t, v)) = value(i)?;
    let (i, _) = newline(i)?;
    Ok((i, (t, v as Value)))
}

pub fn value<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, (node::Tag, Value), E>
{
    let (i, t) = node::decode::tag(i)?;
    let (i, _) = space0(i)?;
    let (i, v) = double(i)?;
    Ok((i, (t, v as Value)))
}
