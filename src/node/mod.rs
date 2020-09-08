mod parser;
pub use parser::decode;

use std::collections::HashMap;

const MAX_INTEGER_TAGS: usize = 4;
const MAX_REAL_TAGS: usize = 4;
const MAX_STRING_TAGS: usize = 2;

pub(crate) type Coordinate = f64;
pub type Dimension = i32;
pub type Tag = i32;

pub type StringTag = String;
pub type RealTag = f64;
pub type IntegerTag = i32;
pub type Value = f64;

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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DataTag {
    Text(StringTag),
    Real(RealTag),
    Integer(IntegerTag),
}

impl Into<Option<StringTag>> for DataTag {
    fn into(self) -> Option<StringTag> {
        match self {
            DataTag::Text(text) => Some(text),
            _ => None,
        }
    }
}

impl Into<Option<RealTag>> for DataTag {
    fn into(self) -> Option<RealTag> {
        match self {
            DataTag::Real(value) => Some(value),
            _ => None,
        }
    }
}

impl Into<Option<IntegerTag>> for DataTag {
    fn into(self) -> Option<IntegerTag> {
        match self {
            DataTag::Integer(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    string_tags     : StringTags,
    real_tags       : RealTags,
    integer_tags    : IntegerTags,
    values          : HashMap<Tag, Value>,
}

impl Data {
    pub fn new(
        string_tags     : StringTags,
        real_tags       : RealTags,
        integer_tags    : IntegerTags,
        values          : HashMap<Tag, Value>
        )
        -> Self
    {
        Self{string_tags, real_tags, integer_tags, values}
    }
}

impl std::default::Default for Data {
    fn default() -> Self {
        Self{
            string_tags     : StringTags::default(),
            real_tags       : RealTags::default(),
            integer_tags    : IntegerTags::default(),
            values          : HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringTags {
    view                    : DataTag,
    interpolation_scheme    : DataTag,
}

impl StringTags{
    pub fn new(view: DataTag, interpolation_scheme: DataTag)
        -> Self
    {
        Self{view, interpolation_scheme}
    }
}

impl std::default::Default for StringTags {
    fn default() -> Self {
        Self{
            view                    : DataTag::Text(String::default()),
            interpolation_scheme    : DataTag::Text(String::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RealTags {
    time_value: DataTag,
}

impl RealTags{
    pub fn new(time_value: DataTag)
        -> Self
    {
        Self{time_value}
    }
}

impl std::default::Default for RealTags {
    fn default() -> Self {
        Self{time_value: DataTag::Real(f64::default())}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerTags {
    time_step_index             : DataTag,
    number_of_field_components  : DataTag,
    number_of_entities          : DataTag,
    partition_index             : DataTag
}

impl IntegerTags{
    pub fn new(
        time_step_index             : DataTag,
        number_of_field_components  : DataTag,
        number_of_entities          : DataTag,
        partition_index             : DataTag
        )
        -> Self
    {
        Self{
            time_step_index,
            number_of_field_components,
            number_of_entities,
            partition_index,
        }
    }
}

impl std::default::Default for IntegerTags {
    fn default() -> Self {
        Self{
            time_step_index             : DataTag::Integer(IntegerTag::default()),
            number_of_field_components  : DataTag::Integer(IntegerTag::default()),
            number_of_entities          : DataTag::Integer(IntegerTag::default()),
            partition_index             : DataTag::Integer(IntegerTag::default()),
        }
    }
}
