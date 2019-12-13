use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut fuel_required = 0;
    for line in contents.lines() {
        let input: i32 = line.parse().unwrap();
        fuel_required += input / 3 - 2;
    }
    println!("Fuel Required: {}", fuel_required);
    Ok(())
}