use std::fmt;

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn from(other: Point) -> Self {
        Point(other.0, other.1)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
