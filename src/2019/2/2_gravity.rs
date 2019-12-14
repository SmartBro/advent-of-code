use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let commands_origin: Vec<usize> = contents
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let output = 19690720;
    for i in 0..99 {
        for j in 0..99 {
            let mut commands = commands_origin.clone();

            commands[1] = i;
            commands[2] = j;

            let mut index = 0;
            while index < commands.len() - 3 {
                let command = commands[index];
                let noun = commands[commands[index + 1]];
                let verb = commands[commands[index + 2]];
                let store_index = commands[index + 3];
                match command {
                    99 => break,
                    1 => commands[store_index] = noun + verb,
                    2 => commands[store_index] = noun * verb,
                    _ => println!("Don't match anything: {}", command),
                }
                index += 4;
            }
            if commands[0] == output {
                println!("Result: {}", 100 * i + j);
                return Ok(());
            }
        }
    }
    Ok(())
}