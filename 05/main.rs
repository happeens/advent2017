use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let lines = contents.split("\n");
    let mut instructions_first: Vec<i32> = lines
        .filter(|line| !line.is_empty())
        .map(|instruction| instruction.parse::<i32>().unwrap())
        .collect();
    let mut instructions_second = instructions_first.clone();

    let mut current_pos = 0;
    let mut steps = 0;

    loop {
        steps += 1;

        let current_instruction = instructions_first[current_pos];
        instructions_first[current_pos] += 1;

        let next_pos = current_pos as i32 + current_instruction;
        if next_pos < 0 || next_pos >= instructions_first.len() as i32 { break; }

        current_pos = next_pos as usize;
    }

    println!("steps needed to leave list on first phase: {}", steps);

    let mut current_pos = 0;
    let mut steps = 0;

    loop {
        steps += 1;

        let current_instruction = instructions_second[current_pos];
        if current_instruction >= 3 {
            instructions_second[current_pos] -= 1;
        }
        else {
            instructions_second[current_pos] += 1;
        }

        let next_pos = current_pos as i32 + current_instruction;
        if next_pos < 0 || next_pos >= instructions_second.len() as i32 { break; }

        current_pos = next_pos as usize;
    }

    println!("steps needed to leave list on second phase: {}", steps);
}
