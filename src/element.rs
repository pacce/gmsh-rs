use crate::node;

pub(crate) type Id = i32;
pub(crate) type Physical = i32;
pub(crate) type Elementary = i32;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Topology {
    Line2(node::Id, node::Id),
    Triangle3(node::Id, node::Id, node::Id),
    Quadrangle4(node::Id, node::Id, node::Id, node::Id),
    Tetrahedron4(node::Id, node::Id, node::Id, node::Id),
    Hexahedron8(
        node::Id,
        node::Id,
        node::Id,
        node::Id,
        node::Id,
        node::Id,
        node::Id,
        node::Id,
    ),
    Prism6(node::Id, node::Id, node::Id, node::Id, node::Id, node::Id),
    Pyramid5(node::Id, node::Id, node::Id, node::Id, node::Id),
    Point1(node::Id),
}
