use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub struct Input(String);

impl Input {
    pub fn read(name: &str) -> Input {
        let mut input = File::open(
            format!("assets/{}.txt", name)
        ).expect("input not found");

        let mut result = String::new();
        input
            .read_to_string(&mut result)
            .expect("could not read input");

        Input(result)
    }

    pub fn into_digits(&self) -> Vec<u32> {
        let &Input(ref input) = self;

        input.chars()
            .filter(|it| it.is_digit(10))
            .map(|it| it.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    }

    pub fn into_lines(&self) -> Vec<String> {
        let &Input(ref input) = self;

        input.split("\n")
            .filter(|it| !it.is_empty())
            .map(|it| it.trim().to_owned())
            .collect::<Vec<String>>()
    }

    pub fn into_list<T>(&self, sep: &str) -> Vec<T>
        where T : FromStr {
        let &Input(ref input) = self;

        input.split(sep)
            .filter(|it| !it.is_empty())
            .map(|it| it.trim())
            .filter_map(|it| match it.parse::<T>() {
                Ok(it) => Some(it),
                Err(_) => None,
            })
            .collect::<Vec<T>>()
    }

    pub fn into_strings(&self, sep: &str) -> Vec<&str> {
        let &Input(ref input) = self;

        input.split(sep)
            .filter(|it| !it.is_empty())
            .map(|it| it.trim())
            .collect::<Vec<&str>>()
    }

    pub fn into_string(&self) -> String {
        let &Input(ref result) = self;
        result.trim().to_owned()
    }
}
