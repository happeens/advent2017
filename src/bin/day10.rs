extern crate advent2017;
use advent2017::file::Input;
use advent2017::knot;

fn main() {
    let nums = Input::read("day10")
        .into_list::<u16>(",");

    let hashed = knot::hash_nums(&nums);
    let result = hashed[0] * hashed[1];
    println!("result: {}", result);

    let input = Input::read("day10").into_string();
    let hashed = knot::hash_str(&input);

    println!("hashed: {}", hashed);
}
