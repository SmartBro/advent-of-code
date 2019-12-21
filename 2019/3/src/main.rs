extern crate common;

mod point;
mod segment;
mod wire;

use common::file::*;
use wire::Wire;
use segment::Segment;
use point::Point;

fn main() -> std::io::Result<()> {
    let contents = read_file(String::from("input.txt"))?;
    let mut lines = contents.lines();
    let starting_point = Point::starting_point();
    let other_point = Point { x: 5, y: 3};
    let mut wires: [Wire; 2] = [
        Wire::new(lines.next().unwrap()),
        Wire::new(lines.next().unwrap()),
    ];

    println!("Part 1 result: {}", wires[0].clone().intersects(wires[1].clone()).len());
    println!("Part 1 result: {}", starting_point.manhattan_distance(other_point));
    Ok(())
}
