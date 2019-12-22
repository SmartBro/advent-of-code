use std::fmt;
use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Segment(pub Point, pub Point);

impl Segment {
    pub fn intersects(self, other: Segment) -> Option<Point> {
        let are_parallel = self.vertical() && other.vertical() && self.0.x != other.0.x
            || !self.vertical() && !other.vertical() && self.0.y != other.0.y;
        if are_parallel {
            return None;
        }
        if self.vertical() && other.in_x_range(self.0) && self.in_y_range(other.0)
            || other.vertical() && self.in_x_range(other.0) && other.in_y_range(self.0) {
            return Some(self.intersection_point(other));
        }
        None
    }

    pub fn intersection_point(self, other: Segment) -> Point {
        let x: i32 = if self.vertical() { self.0.x } else { other.0.x };
        let y: i32 = if self.vertical() { other.0.y } else { self.0.y };

        Point { x, y }
    }

    pub fn vertical(self) -> bool {
        self.0.x == self.1.x
    }

    pub fn in_x_range(self, other: Point) -> bool {
        (self.0.x >= other.x && self.1.x <= other.x) || (self.1.x >= other.x && self.0.x <= other.x)
    }

    pub fn in_y_range(self, other: Point) -> bool {
        (self.0.y >= other.y && self.1.y <= other.y) || (self.1.y >= other.y && self.0.y <= other.y)
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}
