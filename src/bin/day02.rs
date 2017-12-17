extern crate advent2017;
use advent2017::file::Input;

use std::cmp;

fn main() {
    let lines = Input::read("day02")
        .into_lines().iter()
        .map(|it| {
            it.split("\t")
                .filter_map(|it| match it.parse::<u32>() {
                    Ok(it) => Some(it),
                    Err(_) => None,
                })
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

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
