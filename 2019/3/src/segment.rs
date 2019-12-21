use std::fmt;
use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Segment(pub Point, pub Point);

impl Segment {
    pub fn intersects(self, other: Segment) -> Point {
        //TODO: implement intersection logic
        Point::starting_point()
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}
