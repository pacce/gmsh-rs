use {
    crate::node,
    nom::error::ErrorKind,
    std::collections::HashMap,
    super::{
        decode,
        Entity,
    }
};

#[test]
fn entity() {
    let content = "\
2 1 0 6
1
2
3
4
5
6
0. 0. 0.
1. 0. 0.
1. 1. 0.
0. 1. 0.
2. 0. 0.
2. 1. 0.
";
    let mut nodes : HashMap<node::Tag, node::Node> = HashMap::new();
    nodes.insert(1, node::Node::new(0., 0., 0.));
    nodes.insert(2, node::Node::new(1., 0., 0.));
    nodes.insert(3, node::Node::new(1., 1., 0.));
    nodes.insert(4, node::Node::new(0., 1., 0.));
    nodes.insert(5, node::Node::new(2., 0., 0.));
    nodes.insert(6, node::Node::new(2., 1., 0.));

    let expected = Entity{
        dimension   : 2,
        tag         : 1,
        nodes,
    };
    match decode::entity::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}
