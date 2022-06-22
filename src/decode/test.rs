use {
    crate::{
        element::Topology,
        format::Format,
        mesh::{self, Mesh},
        node::Node,
    },
    nom::error::ErrorKind,
    std::io::Cursor,
};

// Test meshes were generated from a *.geo file
// The file has the following implementation
//
// lc = 1.0;
//
// tau = 2*Pi;
//
// electrodes  = 8;
// radius      = 1.0;
//
// z = 0.0;
// Point(1) = {0.0, 0.0, z, lc};
//
// start = newp;
// For i In {0:electrodes - 1}
//     angle = (i * tau) / electrodes;
//     x = radius * Sin (angle);
//     y = radius * Cos (angle);
//     Point(newp) = {x, y, z, lc};
// EndFor
// end = newp - 1;
//
// For c In {start:end}
//     If (c == end)
//         Circle(newl) = {c, 1, start};
//     Else
//         Circle(newl) = {c, 1, c + 1};
//     EndIf
// EndFor
//
// Line Loop(5)        = {1:electrodes};
// Plane Surface(6)    = {5};
// Physical Surface(7) = {5};
// Physical Line(8)    = {5};
//
// Mesh 2;
// Save "disk.msh";

// Generate version 1 gmsh specification from *.geo
// gmsh disk.geo -save_all -format msh1 -

#[test]
fn gmsh1() {
    let text = "\
$NOD
13
1 0 0 0
2 0 1 0
3 0.7071067811865475 0.7071067811865476 0
4 1 6.123233995736766e-17 0
5 0.7071067811865476 -0.7071067811865475 0
6 1.224646799147353e-16 -1 0
7 -0.7071067811865475 -0.7071067811865477 0
8 -1 -1.83697019872103e-16 0
9 -0.7071067811865477 0.7071067811865474 0
10 -0.1113264466680611 -0.2687658173968382 0
11 0.150474547846242 0.3632776942023567 0
12 0.4510798725986296 -0.1820395192602642 0
13 -0.447683015284127 0.1902402582583829 -0
$ENDNOD
$ELM
31
1 15 0 1 1 1
2 15 0 2 1 2
3 15 0 3 1 3
4 15 0 4 1 4
5 15 0 5 1 5
6 15 0 6 1 6
7 15 0 7 1 7
8 15 0 8 1 8
9 15 0 9 1 9
10 1 0 1 2 2 3
11 1 0 2 2 3 4
12 1 0 3 2 4 5
13 1 0 4 2 5 6
14 1 0 5 2 6 7
15 1 0 6 2 7 8
16 1 0 7 2 8 9
17 1 0 8 2 9 2
18 2 0 6 3 9 2 13
19 2 0 6 3 3 4 12
20 2 0 6 3 5 6 12
21 2 0 6 3 6 10 12
22 2 0 6 3 7 8 13
23 2 0 6 3 10 7 13
24 2 0 6 3 2 11 13
25 2 0 6 3 11 3 12
26 2 0 6 3 6 7 10
27 2 0 6 3 2 3 11
28 2 0 6 3 4 5 12
29 2 0 6 3 8 9 13
30 2 0 6 3 11 10 13
31 2 0 6 3 10 11 12
$ENDELM";

    let mut ns = mesh::Nodes::new();
    ns.insert(1, Node::new(0.0, 0.0, 0.0));
    ns.insert(2, Node::new(0.0, 1.0, 0.0));
    ns.insert(3, Node::new(0.7071067811865475, 0.7071067811865476, 0.0));
    ns.insert(4, Node::new(1.0, 6.123233995736766e-17, 0.0));
    ns.insert(5, Node::new(0.7071067811865476, -0.7071067811865475, 0.0));
    ns.insert(6, Node::new(1.224646799147353e-16, -1.0, 0.0));
    ns.insert(7, Node::new(-0.7071067811865475, -0.7071067811865477, 0.0));
    ns.insert(8, Node::new(-1.0, -1.83697019872103e-16, 0.0));
    ns.insert(9, Node::new(-0.7071067811865477, 0.7071067811865474, 0.0));
    ns.insert(10, Node::new(-0.1113264466680611, -0.2687658173968382, 0.0));
    ns.insert(11, Node::new(0.150474547846242, 0.3632776942023567, 0.0));
    ns.insert(12, Node::new(0.4510798725986296, -0.1820395192602642, 0.0));
    ns.insert(13, Node::new(-0.447683015284127, 0.1902402582583829, -0.0));

    let mut es = mesh::Elements::new();

    es.insert(1, (0, 1, Topology::Point1(1)));
    es.insert(2, (0, 2, Topology::Point1(2)));
    es.insert(3, (0, 3, Topology::Point1(3)));
    es.insert(4, (0, 4, Topology::Point1(4)));
    es.insert(5, (0, 5, Topology::Point1(5)));
    es.insert(6, (0, 6, Topology::Point1(6)));
    es.insert(7, (0, 7, Topology::Point1(7)));
    es.insert(8, (0, 8, Topology::Point1(8)));
    es.insert(9, (0, 9, Topology::Point1(9)));

    es.insert(10, (0, 1, Topology::Line2(2, 3)));
    es.insert(11, (0, 2, Topology::Line2(3, 4)));
    es.insert(12, (0, 3, Topology::Line2(4, 5)));
    es.insert(13, (0, 4, Topology::Line2(5, 6)));
    es.insert(14, (0, 5, Topology::Line2(6, 7)));
    es.insert(15, (0, 6, Topology::Line2(7, 8)));
    es.insert(16, (0, 7, Topology::Line2(8, 9)));
    es.insert(17, (0, 8, Topology::Line2(9, 2)));

    es.insert(18, (0, 6, Topology::Triangle3(9, 2, 13)));
    es.insert(19, (0, 6, Topology::Triangle3(3, 4, 12)));
    es.insert(20, (0, 6, Topology::Triangle3(5, 6, 12)));
    es.insert(21, (0, 6, Topology::Triangle3(6, 10, 12)));
    es.insert(22, (0, 6, Topology::Triangle3(7, 8, 13)));
    es.insert(23, (0, 6, Topology::Triangle3(10, 7, 13)));
    es.insert(24, (0, 6, Topology::Triangle3(2, 11, 13)));
    es.insert(25, (0, 6, Topology::Triangle3(11, 3, 12)));
    es.insert(26, (0, 6, Topology::Triangle3(6, 7, 10)));
    es.insert(27, (0, 6, Topology::Triangle3(2, 3, 11)));
    es.insert(28, (0, 6, Topology::Triangle3(4, 5, 12)));
    es.insert(29, (0, 6, Topology::Triangle3(8, 9, 13)));
    es.insert(30, (0, 6, Topology::Triangle3(11, 10, 13)));
    es.insert(31, (0, 6, Topology::Triangle3(10, 11, 12)));

    let expected = Mesh::new(None, ns, es);

    match super::v1::mesh::<(&str, ErrorKind)>(text) {
        Err(_) => assert!(false),
        Ok((_, actual)) => assert_eq!(actual, expected),
    }

    match super::mesh::<(&str, ErrorKind)>(text) {
        Err(_) => assert!(false),
        Ok((_, actual)) => assert_eq!(actual, expected),
    }

    let mut cursor = Cursor::new(text);
    match Mesh::decode(&mut cursor) {
        Err(_) => assert!(false),
        Ok(actual) => assert_eq!(actual, expected),
    }
}

#[test]
fn gmsh2() {
    let text = "\
$MeshFormat
2.2 0 8
$EndMeshFormat
$Nodes
13
1 0 0 0
2 0 1 0
3 0.7071067811865475 0.7071067811865476 0
4 1 0.0 0
5 0.7071067811865476 -0.7071067811865475 0
6 0.0 -1 0
7 -0.7071067811865475 -0.7071067811865477 0
8 -1 0.0 0
9 -0.7071067811865477 0.7071067811865474 0
10 -0.1113264466680611 -0.2687658173968382 0
11 0.150474547846242 0.3632776942023567 0
12 0.4510798725986296 -0.1820395192602642 0
13 -0.447683015284127 0.1902402582583829 -0
$EndNodes
$Elements
31
1 15 2 0 1 1
2 15 2 0 2 2
3 15 2 0 3 3
4 15 2 0 4 4
5 15 2 0 5 5
6 15 2 0 6 6
7 15 2 0 7 7
8 15 2 0 8 8
9 15 2 0 9 9
10 1 2 0 1 2 3
11 1 2 0 2 3 4
12 1 2 0 3 4 5
13 1 2 0 4 5 6
14 1 2 0 5 6 7
15 1 2 0 6 7 8
16 1 2 0 7 8 9
17 1 2 0 8 9 2
18 2 2 0 6 9 2 13
19 2 2 0 6 3 4 12
20 2 2 0 6 5 6 12
21 2 2 0 6 6 10 12
22 2 2 0 6 7 8 13
23 2 2 0 6 10 7 13
24 2 2 0 6 2 11 13
25 2 2 0 6 11 3 12
26 2 2 0 6 6 7 10
27 2 2 0 6 2 3 11
28 2 2 0 6 4 5 12
29 2 2 0 6 8 9 13
30 2 2 0 6 11 10 13
31 2 2 0 6 10 11 12
$EndElements";
    let mut ns = mesh::Nodes::new();

    ns.insert(1, Node::new(0.0, 0.0, 0.0));
    ns.insert(2, Node::new(0.0, 1.0, 0.0));
    ns.insert(3, Node::new(0.7071067811865475, 0.7071067811865476, 0.0));
    ns.insert(4, Node::new(1.0, 0.0, 0.0));
    ns.insert(5, Node::new(0.7071067811865476, -0.7071067811865475, 0.0));
    ns.insert(6, Node::new(0.0, -1.0, 0.0));
    ns.insert(7, Node::new(-0.7071067811865475, -0.7071067811865477, 0.0));
    ns.insert(8, Node::new(-1.0, 0.0, 0.0));
    ns.insert(9, Node::new(-0.7071067811865477, 0.7071067811865474, 0.0));
    ns.insert(10, Node::new(-0.1113264466680611, -0.2687658173968382, 0.0));
    ns.insert(11, Node::new(0.150474547846242, 0.3632776942023567, 0.0));
    ns.insert(12, Node::new(0.4510798725986296, -0.1820395192602642, 0.0));
    ns.insert(13, Node::new(-0.447683015284127, 0.1902402582583829, -0.0));

    let mut es = mesh::Elements::new();

    es.insert(1, (0, 1, Topology::Point1(1)));
    es.insert(2, (0, 2, Topology::Point1(2)));
    es.insert(3, (0, 3, Topology::Point1(3)));
    es.insert(4, (0, 4, Topology::Point1(4)));
    es.insert(5, (0, 5, Topology::Point1(5)));
    es.insert(6, (0, 6, Topology::Point1(6)));
    es.insert(7, (0, 7, Topology::Point1(7)));
    es.insert(8, (0, 8, Topology::Point1(8)));
    es.insert(9, (0, 9, Topology::Point1(9)));

    es.insert(10, (0, 1, Topology::Line2(2, 3)));
    es.insert(11, (0, 2, Topology::Line2(3, 4)));
    es.insert(12, (0, 3, Topology::Line2(4, 5)));
    es.insert(13, (0, 4, Topology::Line2(5, 6)));
    es.insert(14, (0, 5, Topology::Line2(6, 7)));
    es.insert(15, (0, 6, Topology::Line2(7, 8)));
    es.insert(16, (0, 7, Topology::Line2(8, 9)));
    es.insert(17, (0, 8, Topology::Line2(9, 2)));

    es.insert(18, (0, 6, Topology::Triangle3(9, 2, 13)));
    es.insert(19, (0, 6, Topology::Triangle3(3, 4, 12)));
    es.insert(20, (0, 6, Topology::Triangle3(5, 6, 12)));
    es.insert(21, (0, 6, Topology::Triangle3(6, 10, 12)));
    es.insert(22, (0, 6, Topology::Triangle3(7, 8, 13)));
    es.insert(23, (0, 6, Topology::Triangle3(10, 7, 13)));
    es.insert(24, (0, 6, Topology::Triangle3(2, 11, 13)));
    es.insert(25, (0, 6, Topology::Triangle3(11, 3, 12)));
    es.insert(26, (0, 6, Topology::Triangle3(6, 7, 10)));
    es.insert(27, (0, 6, Topology::Triangle3(2, 3, 11)));
    es.insert(28, (0, 6, Topology::Triangle3(4, 5, 12)));
    es.insert(29, (0, 6, Topology::Triangle3(8, 9, 13)));
    es.insert(30, (0, 6, Topology::Triangle3(11, 10, 13)));
    es.insert(31, (0, 6, Topology::Triangle3(10, 11, 12)));

    let f = Format::new(2.2, 0, 8);
    let expected = Mesh::new(Some(f), ns, es);

    match super::v2::mesh::<(&str, ErrorKind)>(text) {
        Err(_) => assert!(false),
        Ok((_, actual)) => assert_eq!(actual, expected),
    }

    match super::mesh::<(&str, ErrorKind)>(text) {
        Err(_) => assert!(false),
        Ok((_, actual)) => assert_eq!(actual, expected),
    }

    let mut cursor = Cursor::new(text);
    match Mesh::decode(&mut cursor) {
        Err(_) => assert!(false),
        Ok(actual) => assert_eq!(actual, expected),
    }
}

#[test]
fn gmsh4() {
    let text = include_str!("t4.msh");

    super::v4::mesh::<(&str, ErrorKind)>(text).unwrap();

    super::mesh::<(&str, ErrorKind)>(text).unwrap();

    let mut cursor = Cursor::new(text);
    Mesh::decode(&mut cursor).unwrap();
}
