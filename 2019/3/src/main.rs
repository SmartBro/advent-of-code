extern crate common;

mod point;
mod segment;
mod wire;

use common::file::*;
use wire::Wire;
use point::Point;

fn main() -> std::io::Result<()> {
    let contents = read_file(String::from("input.txt"))?;
    let mut lines = contents.lines();
    let starting_point = Point::starting_point();
    let first_wire = Wire::new(lines.next().unwrap());
    let second_wire = Wire::new(lines.next().unwrap());
    let intersections = first_wire.clone().intersects(second_wire.clone());

    println!("Part 1 result: {}", starting_point.closest_distance(intersections));
    println!("Part 2 result: {}", first_wire.clone().closest_intersection(second_wire.clone()));
    Ok(())
}
