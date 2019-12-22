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
            match intersection {
                Some(inner) => {
                    if !inner.is_zero() {
                        intersections.push(inner);
                    }
                },
                None => {}
            }
        }
        intersections
    }

    pub fn intersects_steps(self, other: Wire) -> Vec<(Point, u32)> {
        let mut all_intersections: Vec<(Point, u32)> = vec![];
        let mut distance: u32 = 0;
        for segment in self.segments {
            let intersections = other.clone().intersects_segment_steps(segment);
            for mut intersection in intersections {
                let dist = segment.0.manhattan_distance(intersection.0) + distance;
                intersection.1 += dist;
                all_intersections.push(intersection);
            }
            distance += segment.len();
        }

        all_intersections
    }

    pub fn intersects_segment_steps(self, another: Segment) -> Vec<(Point, u32)> {
        let mut intersections: Vec<(Point, u32)> = vec![];
        let mut distance: u32 = 0;
        for segment in self.segments {
            let intersection = segment.intersects(another);
            match intersection {
                Some(inner) => {
                    if !inner.is_zero() {
                        let dist = segment.0.manhattan_distance(inner) + distance;
                        intersections.push((inner, dist));
                    }
                },
                None => {}
            }
            distance += segment.len();
        }
        intersections
    }

    pub fn closest_intersection(self, other: Wire) -> u32 {
        let intersections = self.intersects_steps(other);
        let mut min_dist: u32 = intersections[0].1;
        for intersection in intersections {
            if min_dist > intersection.1 {
                min_dist = intersection.1;
            }
        }
        min_dist
    }
}

fn parse_move(m: &str) -> (char, u16) {
    (m.chars().next().unwrap(), m[1..].parse::<u16>().unwrap())
}
