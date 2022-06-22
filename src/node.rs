pub(crate) type Id = i32;
pub(crate) type Coordinate = f64;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Node {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Node {
    pub const fn new(x: Coordinate, y: Coordinate, z: Coordinate) -> Self {
        Self { x, y, z }
    }
}
