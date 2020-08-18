mod parser;
pub use parser::decode;

use std::collections::HashMap;

pub(crate) type Coordinate = f64;
pub type Dimension = i32;
pub type Tag = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    dimension   : Dimension,
    tag         : Tag,
    nodes       : HashMap<Tag, Node>,
}

impl Entity {
    pub fn new(dimension: Dimension, tag: Tag, nodes: HashMap<Tag, Node>)
        -> Self
    {
        Self{dimension, tag, nodes}
    }
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Node {
    pub fn new(x: Coordinate, y: Coordinate, z: Coordinate) -> Self {
        Self{x, y, z}
    }
}

impl std::default::Default for Node {
    fn default() -> Self {
        Self{
            x: Coordinate::default(),
            y: Coordinate::default(),
            z: Coordinate::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Nodes {
    min         : Tag,
    max         : Tag,
    entities    : Vec<Entity>,
}

impl Nodes {
    pub fn new(min: Tag, max: Tag, entities: Vec<Entity>) -> Self {
        Self{min, max, entities}
    }
}
