use {
    nom::{
        character::complete::newline,
        multi::separated_list
    },
    super::*
};

#[test]
fn default() {
    let expected = Node::default();
    match decode::<(&str, nom::error::ErrorKind)>("0. 0. 0.") {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn multiple() {
    let content = "\
0. 0. 0.
1. 0. 0.
1. 1. 0.
0. 1. 0.
2. 0. 0.
2. 1. 0.";
    let expected = vec![
        Node::new(0.0, 0.0, 0.0),
        Node::new(1.0, 0.0, 0.0),
        Node::new(1.0, 1.0, 0.0),
        Node::new(0.0, 1.0, 0.0),
        Node::new(2.0, 0.0, 0.0),
        Node::new(2.0, 1.0, 0.0),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<Node>> {
        separated_list(newline, decode)(s)
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
