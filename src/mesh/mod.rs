use {
    crate::{
        element::{Elementary, Physical, Topology, self},
        node::{Node, self},
    },
    std::collections::HashMap,
};

pub(crate) type Nodes       = HashMap<node::Id, Node>;
pub(crate) type Elements    = HashMap<element::Id, (Physical, Elementary, Topology)>;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    nodes   : Nodes,
    elements: Elements,
}

impl Mesh {
    pub fn new(nodes: Nodes, elements: Elements) -> Self {
        Self {nodes, elements}
    }
}
