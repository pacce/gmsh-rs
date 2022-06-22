#[derive(Clone, Debug, PartialEq)]
pub struct Integer(i32);

impl Integer {
    fn new(x: i32) -> Self {
        Self(x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    view                    : String,
    interpolation_scheme    : String,
}

impl Text {
    pub fn new(view: String, interpolation_scheme: String) -> Self {
        Self {view, interpolation_scheme}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Real(f64);

impl Real {
    pub fn new(x: f64) -> Self {
        Self(x)
    }
}
