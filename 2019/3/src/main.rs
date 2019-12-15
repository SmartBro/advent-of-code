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
    let mut wires: [Wire; 2] = [
        Wire::new(lines.next().unwrap()),
        Wire::new(lines.next().unwrap()),
    ];

    println!("Part 1 result: {}", wires[0].segments[0]);
    Ok(())
}
