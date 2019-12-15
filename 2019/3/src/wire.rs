use std::mem;
use crate::point::Point;
use crate::segment::Segment;

pub struct Wire {
    pub segments: Vec<Segment>,
}

impl Wire {
    pub fn new(input: &str) -> Self {
        let steps: Vec<(char, u16)> = input.split(",").map(|m| parse_move(m)).collect();
        let mut segments: Vec<Segment> = vec![];
        let mut current = Point(0, 0);
        for step in steps.iter() {
            let prev = Point::from(current);
            let distance: i32 = i32::from(step.1);
            match step.0 {
                'L' => current.0 -= distance,
                'R' => current.0 += distance,
                'U' => current.1 += distance,
                'D' => current.1 -= distance,
                _ => println!("Wrong move {:?}", step),
            }
            segments.push(Segment(prev, current));
        }
        Wire {
            segments,
        }
    }
}

fn parse_move(m: &str) -> (char, u16) {
    (m.chars().next().unwrap(), m[1..].parse::<u16>().unwrap())
}
