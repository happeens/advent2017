extern crate advent2017;
use advent2017::file::Input;

fn main() {
    let mut blocks = Input::read("day06")
        .into_list::<u32>(" ");

    let mut known_cycles = vec![hash_cycle(&blocks)];
    let mut steps = 0;

    loop {
        steps += 1;

        let current_max = blocks.iter().cloned().max().unwrap();
        let max_index = blocks.iter().position(|&i| i == current_max).unwrap();

        redistribute(&mut blocks, max_index);

        let hash = hash_cycle(&blocks);

        if known_cycles.contains(&hash) {
            println!("known cycle found after {} iterations", steps);

            let last_known_pos = known_cycles.iter()
                .position(|ref h| &hash == *h)
                .unwrap();

            println!("cycle size is {}", steps - last_known_pos);

            break;
        }

        known_cycles.push(hash);
    }
}

fn redistribute(v: &mut Vec<u32>, i: usize) {
    let mut x = v[i];
    v[i] = 0;
    let mut i = i;
    i = i % v.len();

    while x > 0 {
        i = (i + 1) % v.len();
        v[i] += 1;
        x -= 1;
    }
}

fn hash_cycle(v: &Vec<u32>) -> String {
    let strings: Vec<String> = v.iter()
        .map(|i| i.to_string())
        .collect();

    let mut result = String::new();
    for s in &strings {
        result.push_str(s);
        result.push('.');
    }

    result.pop();

    result
}
