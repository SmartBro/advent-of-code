use std::fmt;
use crate::point::Point;

pub struct Segment(pub Point, pub Point);

impl Segment {
    pub fn intersects(other: Segment) -> bool {
        false
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}
