use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let lines: Vec<Vec<String>> = contents
        .split("\n")
        .map(|line| {
            let line: Vec<String> = line
                .split(" ")
                .filter(|word| !word.is_empty())
                .map(|word| word.to_owned())
                .collect();

            line
        })
        .filter(|line| !line.is_empty())
        .collect();

    let valid_count = count_valid_lines(&lines);

    println!("valid phrases: {}", valid_count);

    let sorted_lines: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|word| {
                    let mut chars: Vec<char> = word.chars().collect();
                    chars.sort_by(|a, b| a.cmp(b));
                    String::from_iter(chars)
                })
                .collect()
        })
        .collect();

    let valid_count = count_valid_lines(&sorted_lines);

    println!("valid phrases: {}", valid_count);
}

fn count_valid_lines(lines: &Vec<Vec<String>>) -> usize {
    lines
        .iter()
        .filter(|line| {
            let set: HashSet<String> =
                HashSet::from_iter(line.iter().cloned());
            
            set.len() == line.len()
        })
        .count()
}
