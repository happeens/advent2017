use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let numbers: Vec<u32> = contents
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut sum_first = 0;

    for i in 0..numbers.len() {
        let current = access_circular(&numbers, i);
        let next = access_circular(&numbers, i + 1);

        if current == next {
            sum_first += next;
        }
    }

    println!("first sum: {}", sum_first);

    let mut sum_second = 0;
    let advance = numbers.len() / 2;

    for i in 0..numbers.len() {
        let current = access_circular(&numbers, i);
        let next = access_circular(&numbers, i + advance);

        if current == next {
            sum_second += next;
        }
    }

    println!("second sum: {}", sum_second);
}

fn access_circular(v: &Vec<u32>, i: usize) -> u32 {
    let i = i % v.len();
    v[i]
}
