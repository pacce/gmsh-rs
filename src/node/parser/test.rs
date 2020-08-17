use {
    nom::{
        error::ErrorKind,
        multi::many0,
        IResult,
    },
    super::{
        decode::{
            coordinates,
            node,
            tag
        },
        Node,
        Tag,
    }
};

#[test]
fn coordinate() {
    let expected = Node::default();
    match coordinates::<(&str, ErrorKind)>("0. 0. 0.") {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn nodes() {
    let content = "\
0. 0. 0.
1. 0. 0.
1. 1. 0.
0. 1. 0.
2. 0. 0.
2. 1. 0.
";
    let expected = vec![
        Node::new(0.0, 0.0, 0.0),
        Node::new(1.0, 0.0, 0.0),
        Node::new(1.0, 1.0, 0.0),
        Node::new(0.0, 1.0, 0.0),
        Node::new(2.0, 0.0, 0.0),
        Node::new(2.0, 1.0, 0.0),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<Node>> {
        many0(node)(s)
    }
    match parser(content) {
        Ok((_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for (e, a) in expected.iter().zip(actual) {
                assert_eq!(*e, a);
            }
        }
        Err(_) => assert!(false),
    };
}

#[test]
fn tags() {
    let content = "\
1
2
3
4
5
6
";
    let expected = vec![
        1,
        2,
        3,
        4,
        5,
        6,
    ];
    fn parser(s: &str) -> IResult<&str, Vec<Tag>> {
        many0(tag)(s)
    }
    match parser(content) {
        Ok((_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for (e, a) in expected.iter().zip(actual) {
                assert_eq!(*e, a);
            }
        }
        Err(_) => assert!(false),
    };
}
