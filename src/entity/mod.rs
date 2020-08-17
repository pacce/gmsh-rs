mod parser;

use {
    crate::node,
    std::collections::HashMap,
};

pub type Tag = i32;
pub type Dimension = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    dimension   : Dimension,
    tag         : Tag,
    nodes       : HashMap<node::Tag, node::Node>,
}

impl std::default::Default for Entity {
    fn default() -> Self {
        Self {
            dimension   : Dimension::default(),
            tag         : Tag::default(),
            nodes       : HashMap::new()
        }
    }
}
