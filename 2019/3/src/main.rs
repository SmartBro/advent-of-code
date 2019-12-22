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
    let intersections = Wire::new(lines.next().unwrap())
        .intersects(Wire::new(lines.next().unwrap()));

    println!("Part 1 result: {}", starting_point.closest_distance(intersections));
    Ok(())
}
