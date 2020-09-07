use {
    crate::node,
    nom::{
        error::ErrorKind,
        multi::many0,
        IResult,
    },
    std::collections::HashMap,
    super::{
        decode,
        IntegerTags,
        NodeData,
        RealTags,
        RealTag,
        StringTag,
        StringTags,
        Value,
    },
};

#[test]
fn node_data() {
    let content = r#"$NodeData
1
"A scalar view"
1
0.0
3
0
1
6
1 0.0
2 0.1
3 0.2
4 0.0
5 0.2
6 0.4
$EndNodeData
"#;
    let sts = StringTags::new("A scalar view".to_string(), "".to_string());
    let rts = RealTags::new(0.0);
    let its = IntegerTags::new(0, 1, 6, 0);

    let mut vs : HashMap<node::Tag, Value> = HashMap::new();
    vs.insert(1, 0.0);
    vs.insert(2, 0.1);
    vs.insert(3, 0.2);
    vs.insert(4, 0.0);
    vs.insert(5, 0.2);
    vs.insert(6, 0.4);

    let expected = NodeData::new(sts, rts, its, vs);
    match decode::node_data::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(e) => {
            eprintln!("{:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn string_tag() {
    let content = r#"1
"A scalar view"
"#;
    let expected = StringTags::new("A scalar view".to_string(), "".to_string());
    match decode::string_tags::<(&str, ErrorKind)>(content) {
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
1
0.0
";
    let expected = RealTags::new(0.0);
    match decode::real_tags::<(&str, ErrorKind)>(content) {
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
3
0
1
6
";
    let expected = IntegerTags::new(0, 1, 6, 0);
    match decode::integer_tags::<(&str, ErrorKind)>(content) {
        Ok((i_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn values() {
    let content = "\
1 0.0
2 0.1
3 0.2
4 0.0
5 0.2
6 0.4
";
    let expected = vec![
        (1, 0.0),
        (2, 0.1),
        (3, 0.2),
        (4, 0.0),
        (5, 0.2),
        (6, 0.4),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(node::Tag, Value)>> {
        many0(decode::value_newline)(s)
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
