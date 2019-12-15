extern crate common;

use common::file::*;

fn main() -> std::io::Result<()> {
    let contents = read_file(String::from("input.txt"))?;
    let mut fuel_required = 0;
    for line in contents.lines() {
        let input: i32 = line.parse().unwrap();
        fuel_required += calc_fuel(input);
    }
    println!("Part 1 fuel required: {}", fuel_required);

    fuel_required = 0;
    for line in contents.lines() {
        let mut input: i32 = line.parse().unwrap();
        input = calc_fuel(input);
        while input > 0 {
            fuel_required += input;
            input = calc_fuel(input);
        }
    }
    println!("Part 2 fuel required: {}", fuel_required);
    Ok(())
}

fn calc_fuel(input: i32) -> i32 {
    input / 3 - 2
}
