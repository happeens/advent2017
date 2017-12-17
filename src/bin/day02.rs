use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let lines = contents.split("\n");
    let lines: Vec<Vec<u32>> = lines
        .map(|line| {
            line.split("\t")
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .filter(|x| *x != 0u32)
                .collect()
        })
        .collect();

    let mut checksum = 0;

    for line in &lines {
        let mut min = line.first().expect("empty line");
        let mut max = line.first().expect("empty line");

        for n in line {
            min = cmp::min(min, n);
            max = cmp::max(max, n);
        }

        checksum += max - min
    }

    println!("first checksum: {}", checksum);

    let mut checksum = 0;

    for line in &lines {
        for n in line {
            let div: Vec<u32> = line
                .iter()
                .filter(|x| *x < n && n % *x == 0)
                .map(|x| *x)
                .collect();

            if !div.is_empty() {
                checksum += n / div.first().unwrap();
            }
        }
    }

    println!("second checksum: {}", checksum);
}
