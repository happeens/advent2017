use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let contents = contents.trim();
    let tokens = tokenize(&contents);

    let total_score = get_group_sum(&tokens);
    println!("total score: {}", total_score);

    let total_garbage_length = get_total_garbage_length(&tokens);
    println!("total garbage length: {}", total_garbage_length);
}

fn get_group_sum(tokens: &Vec<Token>) -> u32 {
    let mut depth = 0;
    let mut result = 0;

    for it in tokens {
        match *it {
            Token::GroupStart => depth += 1,
            Token::GroupEnd => {
                result += depth;
                depth -= 1;
            },
            Token::Garbage(_) => (),
        }
    }

    result
}

fn get_total_garbage_length(tokens: &Vec<Token>) -> u32 {
    let mut result = 0;

    for it in tokens {
        match *it {
            Token::Garbage(n) => result += n,
            _ => (),
        }
    }

    result
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut reading_garbage = false;
    let mut ignore_next = false;
    let mut garbage_count = 0;

    for it in input.chars() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        if reading_garbage {
            match it {
                '!' => ignore_next = true,
                '>' => {
                    reading_garbage = false;
                    result.push(Token::Garbage(garbage_count));
                },
                _ => garbage_count += 1,
            }
            
            continue;
        }

        match it {
            '{' => result.push(Token::GroupStart),
            '}' => result.push(Token::GroupEnd),
            '<' => {
                reading_garbage = true;
                garbage_count = 0;
            },
            ',' => (),
            _ => panic!("invalid char outside garbage"),
        }
    }

    result
}

#[derive(Debug)]
enum Token {
    GroupStart,
    GroupEnd,
    Garbage(u32),
}
