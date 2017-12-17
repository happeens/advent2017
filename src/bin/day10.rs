use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let nums = contents.split(",");
    let nums = nums
        .filter(|it| !it.is_empty())
        .map(|it| it.trim().parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let list = (0..256).collect::<Vec<u16>>();
    let (hashed, _, _) = hash_list(&nums, &list, 0, 0);
    let result = hashed[0] * hashed[1];
    println!("result: {}", result);

    let mut nums = contents.trim().chars()
        .map(|it| it as u16)
        .collect::<Vec<u16>>();

    for it in [17, 31, 73, 47, 23].iter() {
        nums.push(*it as u16);
    }

    let mut list = (0..256).collect::<Vec<u16>>();
    let mut current_pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        let (new_list, new_pos, new_skip_size) =
            hash_list(&nums, &list, current_pos, skip_size);

        list = new_list;
        current_pos = new_pos;
        skip_size = new_skip_size;
    }

    let mut result = String::new();
    for chunk in list.chunks(16) {
        let mut chunk_result = chunk[0];
        for i in 1..16 {
            chunk_result = chunk_result ^ chunk[i];
        }
        result.push_str(&format!("{:02x}", chunk_result));
    }

    println!("hashed: {}", result);
}

fn hash_list(
    nums: &Vec<u16>,
    list: &Vec<u16>,
    current_pos: usize,
    skip_size: usize
) -> (Vec<u16>, usize, usize) {
    let mut list = list.clone();
    let mut current_pos = current_pos;
    let mut skip_size = skip_size;

    for it in nums {
        let it = *it as usize;

        assert!(it <= list.len());

        let mut slice = get_slice(current_pos, it, &list);
        slice.reverse();

        for i in 0..slice.len() {
            set_circular(
                (i + current_pos) as usize,
                &mut list,
                slice[i]
            );
        }

        current_pos += skip_size + it;
        skip_size += 1;
    }

    (list, current_pos, skip_size)
}

fn access_circular(i: usize, list: &Vec<u16>) -> u16 {
    list[i % list.len()]
}

fn set_circular(i: usize, list: &mut Vec<u16>, v: u16) {
    let i = i % list.len();
    list[i] = v;
}

fn get_slice(i: usize, len: usize, list: &Vec<u16>) -> Vec<u16> {
    let mut result = Vec::new();

    for at in i..(i + len) {
        result.push(access_circular(at, list));
    }

    result
}
