use {
    crate::{
        decode,
        element::{self, Elementary, Physical, Topology},
        format::Format,
        node::{self, Node},
    },
    std::{collections::HashMap, io::Read},
};

pub(crate) type Nodes = HashMap<node::Id, Node>;
pub(crate) type Elements = HashMap<element::Id, (Physical, Elementary, Topology)>;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    format: Option<Format>,
    nodes: Nodes,
    elements: Elements,
}

impl Mesh {
    pub const fn new(format: Option<Format>, nodes: Nodes, elements: Elements) -> Self {
        Self {
            format,
            nodes,
            elements,
        }
    }

    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let mut ss = String::new();
        reader.read_to_string(&mut ss)?;
        decode::mesh::<nom::error::Error<&str>>(&ss)
            .map(|(_, mesh)| mesh)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to decode mesh"))
    }

    pub const fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    pub const fn elements(&self) -> &Elements {
        &self.elements
    }
}
