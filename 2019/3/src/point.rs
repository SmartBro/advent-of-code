use std::fmt;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn from(other: Point) -> Self {
        Point { x: other.x, y: other.y }
    }

    pub fn starting_point() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn is_zero(self) -> bool { self.x == 0 && self.y == 0 }

    pub fn move_self(&mut self, step: (char, u16)) {
        let distance: i32 = i32::from(step.1);
        match step.0 {
            'L' => self.x -= distance,
            'R' => self.x += distance,
            'U' => self.y += distance,
            'D' => self.y -= distance,
            _ => println!("Wrong move {:?}", step),
        }
    }

    pub fn manhattan_distance(self, other: Point)-> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
