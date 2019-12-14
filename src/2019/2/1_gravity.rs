use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut commands: Vec<usize> = contents
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    // 1202 program alarm state
    commands[1] = 12;
    commands[2] = 2;

    let mut index = 0;
    while index < commands.len() - 3 {
        let command = commands[index];
        let store_index = commands[index + 3];
        match command {
            99 => break,
            1 => commands[store_index] = commands[commands[index + 1]] + commands[commands[index + 2]],
            2 => commands[store_index] = commands[commands[index + 1]] * commands[commands[index + 2]],
            _ => println!("Don't match anything: {}", command),
        }
        index += 4;
    }
    println!("Program halts at index: {}", index);
    println!("Result: {}", commands[0]);
    Ok(())
}