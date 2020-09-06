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
