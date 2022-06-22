pub(crate) type Id = i32;
pub(crate) type Coordinate = f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Node {
    pub fn new(x: Coordinate, y: Coordinate, z: Coordinate) -> Self {
        Self { x, y, z }
    }
}

impl std::default::Default for Node {
    fn default() -> Self {
        Self {
            x: Coordinate::default(),
            y: Coordinate::default(),
            z: Coordinate::default(),
        }
    }
}
