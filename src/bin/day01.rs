extern crate advent2017;
use advent2017::file::Input;
use advent2017::circular;

fn main() {
    let numbers = Input::read("day01").into_digits();

    let mut sum_first = 0;

    for i in 0..numbers.len() {
        let current = circular::access_circular(i, &numbers);
        let next = circular::access_circular(i + 1, &numbers);

        if current == next {
            sum_first += next;
        }
    }

    println!("first sum: {}", sum_first);

    let mut sum_second = 0;
    let advance = numbers.len() / 2;

    for i in 0..numbers.len() {
        let current = circular::access_circular(i, &numbers);
        let next = circular::access_circular(i + advance, &numbers);

        if current == next {
            sum_second += next;
        }
    }

    println!("second sum: {}", sum_second);
}
