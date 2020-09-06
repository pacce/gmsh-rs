use {
    nom::{
        error::ErrorKind,
        multi::many0,
        IResult,
    },
    std::collections::HashMap,
    super::{
        decode::{
            self,
        },
        IntegerTag,
        RealTag,
        StringTag,
        Value,
    },
};

#[test]
fn string_tag() {
    let content = r#""A scalar view""#;
    let expected : StringTag = "A scalar view".to_string();
    match decode::string_tag::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(e) => {
            eprintln!("{:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn real_tag() {
    let content = "\
0.0
";
    let expected : RealTag = 0.0;
    match decode::real_tag::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(e) => {
            eprintln!("{:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn integer_tags() {
    let content = "\
0
1
6
";
    let expected = vec![
        0,
        1,
        6
    ];
    fn parser(s: &str) -> IResult<&str, Vec<IntegerTag>> {
        many0(decode::integer_tag_newline)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for (e, a) in expected.iter().zip(actual) {
                assert_eq!(*e, a);
            }
        }
        Err(_) => assert!(false),
    }
}
