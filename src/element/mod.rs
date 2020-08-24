mod parser;
pub use parser::decode;

use std::collections::HashMap;

use crate::node;

pub type Dimension = i32;
pub type ElementTag = usize;
pub type ElementType = i32;
pub type EntityTag = i32;

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
    Line2 ( node::Tag, node::Tag ),
    Triangle3 ( node::Tag, node::Tag, node::Tag ),
}
