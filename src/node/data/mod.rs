mod parser;
pub use parser::decode;

use std::collections::HashMap;

use crate::node;

const MAX_INTEGER_TAGS: usize = 4;
const MAX_REAL_TAGS: usize = 4;
const MAX_STRING_TAGS: usize = 2;

pub type StringTag =String;
pub type RealTag = f64;
pub type IntegerTag = i32;
pub type Value = f64;

#[derive(Clone, Debug, PartialEq)]
pub struct NodeData {
    string_tags     : StringTags,
    real_tags       : RealTags,
    integer_tags    : IntegerTags,
    values          : HashMap<node::Tag, Value>,
}

impl NodeData {
    pub fn new(
        string_tags     : StringTags,
        real_tags       : RealTags,
        integer_tags    : IntegerTags,
        values          : HashMap<node::Tag, Value>
        )
        -> Self
    {
        Self{string_tags, real_tags, integer_tags, values}
    }
}

impl std::default::Default for NodeData {
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
    view                    : StringTag,
    interpolation_scheme    : StringTag,
}

impl StringTags{
    pub fn new(view: StringTag, interpolation_scheme: StringTag)
        -> Self
    {
        Self{view, interpolation_scheme}
    }
}

impl std::default::Default for StringTags {
    fn default() -> Self {
        Self{
            view                    : StringTag::default(),
            interpolation_scheme    : StringTag::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RealTags {
    time_value: RealTag,
}

impl RealTags{
    pub fn new(time_value: RealTag)
        -> Self
    {
        Self{time_value}
    }
}

impl std::default::Default for RealTags {
    fn default() -> Self {
        Self{time_value: RealTag::default()}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerTags {
    time_step_index             : IntegerTag,
    number_of_field_components  : IntegerTag,
    number_of_entities          : IntegerTag,
    partition_index             : IntegerTag
}

impl IntegerTags{
    pub fn new(
        time_step_index             : IntegerTag,
        number_of_field_components  : IntegerTag,
        number_of_entities          : IntegerTag,
        partition_index             : IntegerTag
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
            time_step_index             : IntegerTag::default(),
            number_of_field_components  : IntegerTag::default(),
            number_of_entities          : IntegerTag::default(),
            partition_index             : IntegerTag::default(),
        }
    }
}
