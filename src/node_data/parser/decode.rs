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
