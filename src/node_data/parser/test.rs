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

/*
#[test]
fn elements() {
    let content = "\
$Elements
1 2 1 2
2 1 2 2
1 1 2 3
2 3 4 5
$EndElements
";
    let mut es : HashMap<ElementTag, Element> = HashMap::new();
    es.insert(1, Element::Triangle3(1, 2, 3));
    es.insert(2, Element::Triangle3(3, 4, 5));

    let entities = Entity::new(2, 1, 2, es);
    let expected = Elements::new(1, 2, vec![entities]);
    match decode::elements::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(e) => {
            eprintln!("{:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn entity_line2() {
    let content = "\
2 1 1 2
1 1 2
2 3 4
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Line2(1, 2));
    elements.insert(2, Element::Line2(3, 4));

    let expected = Entity::new(2, 1, 1, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_triangle3() {
    let content = "\
2 1 2 2
1 1 2 3
2 4 5 6
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Triangle3(1, 2, 3));
    elements.insert(2, Element::Triangle3(4, 5, 6));

    let expected = Entity::new(2, 1, 2, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_quadrangle4() {
    let content = "\
2 1 3 2
1 1 2 3 4
2 5 6 7 8
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Quadrangle4(1, 2, 3, 4));
    elements.insert(2, Element::Quadrangle4(5, 6, 7, 8));

    let expected = Entity::new(2, 1, 3, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_tetrahedron4() {
    let content = "\
2 1 4 2
1 1 2 3 4
2 5 6 7 8
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Tetrahedron4(1, 2, 3, 4));
    elements.insert(2, Element::Tetrahedron4(5, 6, 7, 8));

    let expected = Entity::new(2, 1, 4, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_hexahedron8() {
    let content = "\
2 1 5 2
1 1 2 3 4 5 6 7 8
2 9 10 11 12 13 14 15 16
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Hexahedron8(1, 2, 3, 4, 5, 6, 7, 8));
    elements.insert(2, Element::Hexahedron8(9, 10, 11, 12, 13, 14, 15, 16));

    let expected = Entity::new(2, 1, 5, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_prism6() {
    let content = "\
2 1 6 2
1 1 2 3 4 5 6
2 7 8 9 10 11 12
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Prism6(1, 2, 3, 4, 5, 6));
    elements.insert(2, Element::Prism6(7, 8, 9, 10, 11, 12));

    let expected = Entity::new(2, 1, 6, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn entity_pyramid5() {
    let content = "\
2 1 7 2
1 1 2 3 4 5
2 6 7 8 9 10
";
    let mut elements : HashMap<ElementTag, Element> = HashMap::new();
    elements.insert(1, Element::Pyramid5(1, 2, 3, 4, 5));
    elements.insert(2, Element::Pyramid5(6, 7, 8, 9, 10));

    let expected = Entity::new(2, 1, 7, elements);
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_line2() {
    let content = "\
1 1 2
2 3 4
3 5 6
";
    let expected = vec![
        (1, Element::Line2(1, 2)),
        (2, Element::Line2(3, 4)),
        (3, Element::Line2(5, 6)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(line2)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_triangle3() {
    let content = "\
1 1 2 3
2 4 5 6
3 7 8 9
";
    let expected = vec![
        (1, Element::Triangle3(1, 2, 3)),
        (2, Element::Triangle3(4, 5, 6)),
        (3, Element::Triangle3(7, 8, 9)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(triangle3)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_quadrangle4() {
    let content = "\
1 1 2 3 4
2 5 6 7 8
";
    let expected = vec![
        (1, Element::Quadrangle4(1, 2, 3, 4)),
        (2, Element::Quadrangle4(5, 6, 7, 8)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(quadrangle4)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_tetrahedron4() {
    let content = "\
1 1 2 3 4
2 5 6 7 8
";
    let expected = vec![
        (1, Element::Tetrahedron4(1, 2, 3, 4)),
        (2, Element::Tetrahedron4(5, 6, 7, 8)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(tetrahedron4)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_hexahedron8() {
    let content = "\
1 1 2 3 4 5 6 7 8
2 9 10 11 12 13 14 15 16
";
    let expected = vec![
        (1, Element::Hexahedron8(1, 2, 3, 4, 5, 6, 7, 8)),
        (2, Element::Hexahedron8(9, 10, 11, 12, 13, 14, 15, 16)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(hexahedron8)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_prism6() {
    let content = "\
1 1 2 3 4 5 6
2 7 8 9 10 11 12
";
    let expected = vec![
        (1, Element::Prism6(1, 2, 3, 4, 5, 6)),
        (2, Element::Prism6(7, 8, 9, 10, 11, 12)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(prism6)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn elements_pyramid5() {
    let content = "\
1 1 2 3 4 5
2 6 7 8 9 10
";
    let expected = vec![
        (1, Element::Pyramid5(1, 2, 3, 4, 5)),
        (2, Element::Pyramid5(6, 7, 8, 9, 10)),
    ];
    fn parser(s: &str) -> IResult<&str, Vec<(ElementTag, Element)>> {
        many0(pyramid5)(s)
    }
    match parser(content) {
        Ok((i_, actual)) => {
            assert_eq!(expected.len(), actual.len());
            for ((et, ee), (at, ae)) in expected.iter().zip(actual) {
                assert_eq!(*et, at);
                assert_eq!(*ee, ae);
            }
        }
        Err(_) => assert!(false),
    }
}
*/
