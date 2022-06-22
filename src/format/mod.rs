#[derive(Clone, Debug, PartialEq)]
pub struct Format {
    version: f64,
    file: i32,
    size: i32,
}

impl Format {
    pub fn new(version: f64, file: i32, size: i32) -> Self {
        Self {
            version,
            file,
            size,
        }
    }
}
