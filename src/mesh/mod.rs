use {
    crate::{
        element::{Elementary, Physical, Topology, self},
        format::Format,
        node::{Node, self},
    },
    std::collections::HashMap,
};

pub(crate) type Nodes       = HashMap<node::Id, Node>;
pub(crate) type Elements    = HashMap<element::Id, (Physical, Elementary, Topology)>;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    format  : Option<Format>,
    nodes   : Nodes,
    elements: Elements,
}

impl Mesh {
    pub fn new(format: Option<Format>, nodes: Nodes, elements: Elements) -> Self {
        Self {format, nodes, elements}
    }
}
