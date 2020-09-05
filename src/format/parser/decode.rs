use {
    nom::{
        bytes::complete,
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
    },
    super::{
        DataSize,
        FileType,
        Format,
        Version,
    }
};

pub fn format<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Format, E>
{
    let (i, _)      = complete::tag("$MeshFormat")(i)?;
    let (i, _)      = newline(i)?;

    let (i, ver)    = version(i)?;
    let (i, _)      = space0(i)?;
    let (i, ft)     = file_type(i)?;
    let (i, _)      = space0(i)?;
    let (i, ds)     = data_size(i)?;
    let (i, _)      = newline(i)?;    

    let (i, _)      = complete::tag("$EndMeshFormat")(i)?;
    let (i, _)      = newline(i)?;
    
    Ok((i, Format::new(ver, ft, ds)))
}

pub fn version<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, Version, E>
{
    let (i, ver) = float(i)?;
    Ok((i, ver.to_string() as Version))
}

pub fn file_type<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, FileType, E>
{
    let (i, file_type) = float(i)?;
    Ok((i, file_type as FileType))
}

pub fn data_size<'a, E: ParseError<&'a str>>(i: &'a str)
    -> IResult<&'a str, DataSize, E>
{
    let (i, data_size) = float(i)?;
    Ok((i, data_size as DataSize))
}
