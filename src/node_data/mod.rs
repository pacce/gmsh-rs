mod parser;
pub use parser::decode;

use std::collections::HashMap;

use crate::node;

pub type StringTag =String;
pub type RealTag = f64;
pub type IntegerTag = i32;
pub type Value = f64;

#[derive(Clone, Debug, PartialEq)]
pub struct NodeData {
    string_tags     : Vec<StringTag>,
    real_tags       : Vec<RealTag>,
    integer_tags    : Vec<IntegerTag>,
    values          : HashMap<node::Tag, Value>,
}

impl NodeData {
    pub fn new(
        string_tags: Vec<StringTag>,
        real_tags: Vec<RealTag>,
        integer_tags: Vec<IntegerTag>,
        values: HashMap<node::Tag, Value>
        )
        -> Self {
        Self{string_tags, real_tags, integer_tags, values}
    }
}

/*
#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    dimension   : Dimension,
    tag         : EntityTag,
    element_type: ElementType,
    elements    : HashMap<ElementTag, Element>,
}

impl Entity {
    pub fn new(
        dimension   : Dimension,
        tag         : EntityTag,
        element_type: ElementType,
        elements    : HashMap<ElementTag, Element>
        ) -> Self
    {
        Self{dimension, tag, element_type, elements}
    }
}

impl std::default::Default for Entity {
    fn default() -> Self {
        Self {
            dimension   : Dimension::default(),
            tag         : EntityTag::default(),
            element_type: ElementType::default(),
            elements    : HashMap::new()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Element {
    Line2 (
        node::Tag,
        node::Tag
        ),
    Triangle3 (
        node::Tag,
        node::Tag,
        node::Tag
        ),
    Quadrangle4 (
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag
        ),
    Tetrahedron4 (
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag
        ),
    Hexahedron8 (
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        ),
    Prism6 (
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        ),
    Pyramid5 (
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        node::Tag,
        ),
}
*/
