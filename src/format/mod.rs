mod parser;
pub use parser::decode;

pub type Version = String;
pub type FileType = i32;
pub type DataSize = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Format {
    version     : Version,
    file_type   : FileType,
    data_size   : DataSize,
}

impl Format {
    pub fn new(version: Version, file_type: FileType, data_size: DataSize)
        -> Self
    {
        Self{version, file_type, data_size}
    }
}
