extern crate common;

use common::file::*;

fn main() -> std::io::Result<()> {
    let contents = read_file(String::from("input.txt"))?;
    let commands: Vec<usize> = contents
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    println!("Part 1 result: {}", gravity_check(12, 2, commands.clone()));

    let output = 19690720;
    for i in 0..99 {
        for j in 0..99 {
            let result = gravity_check(i, j, commands.clone());
            if result == output {
                println!("Part 2 result: {}", 100 * i + j);
                return Ok(());
            }
        }
    }
    Ok(())
}

fn gravity_check(noun: usize, verb: usize, mut commands: Vec<usize>) -> usize {
    commands[1] = noun;
    commands[2] = verb;

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

    commands[0]
}
