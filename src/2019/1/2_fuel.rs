use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut fuel_required = 0;
    for line in contents.lines() {
        let mut input: i32 = line.parse().unwrap();
        input = calc_fuel(input);
        while input > 0 {
            fuel_required += input;
            input = calc_fuel(input);
        }
    }
    println!("Fuel Required: {}", fuel_required);
    Ok(())
}

fn calc_fuel(input: i32) -> i32 {
    input / 3 - 2
}