use std::mem;
use crate::point::Point;
use crate::segment::Segment;

#[derive(Clone)]
pub struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    pub fn new(input: &str) -> Self {
        let steps: Vec<(char, u16)> = input.split(",").map(|m| parse_move(m)).collect();
        let mut segments: Vec<Segment> = vec![];
        let mut current = Point::starting_point();
        for step in steps.iter() {
            let prev = Point::from(current);
            current.move_self(*step);
            segments.push(Segment(prev, current));
        }
        Wire {
            segments,
        }
    }

    pub fn intersects(self, other: Wire) -> Vec<Point> {
        let mut all_intersections: Vec<Point> = vec![];
        for segment in self.segments {
            let mut intersections = other.clone().intersects_segment(segment);
            all_intersections.append(&mut intersections);
        }

        all_intersections
    }

    pub fn intersects_segment(self, another: Segment) -> Vec<Point> {
        let mut intersections: Vec<Point> = vec![];
        for segment in self.segments {
            let intersection = segment.intersects(another);
            if !intersection.is_zero() {
                intersections.push(intersection);
            }
        }
        intersections
    }
}

fn parse_move(m: &str) -> (char, u16) {
    (m.chars().next().unwrap(), m[1..].parse::<u16>().unwrap())
}
