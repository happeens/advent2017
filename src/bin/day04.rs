extern crate advent2017;
use advent2017::file::Input;

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let lines = Input::read("day04")
        .into_lines().iter()
        .map(|it| {
            it.split(" ")
                .filter_map(|it| match it.is_empty() {
                    false => Some(it.to_owned()),
                    _ => None,
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

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
