extern crate advent2017;
use advent2017::file::Input;

fn main() {
    let lines = Input::read("day05").into_lines();
    let mut instructions_first = lines.iter()
        .filter_map(|it| match it.parse::<i32>() {
            Ok(it) => Some(it),
            Err(_) => None,
        })
        .collect::<Vec<i32>>();

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
