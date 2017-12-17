use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let directions = contents.split(",")
        .filter(|it| !it.is_empty())
        .map(|it| it.trim())
        .map(|it| match it {
            "n" => HexDir::N,
            "ne" => HexDir::NE,
            "nw" => HexDir::NW,
            "s" => HexDir::S,
            "se" => HexDir::SE,
            "sw" => HexDir::SW,
            _ => panic!("invalid direction"),
        })
        .collect::<Vec<HexDir>>();


    let mut pos = HexPos { x: 0, y: 0, z: 0 };
    let mut furthest_distance = 0;
    for it in &directions {
        match *it {
            HexDir::N => {
                pos.y += 1;
                pos.z -= 1;
            },
            HexDir::NE => {
                pos.x += 1;
                pos.z -= 1;
            },
            HexDir::NW => {
                pos.y += 1;
                pos.x -= 1;
            },
            HexDir::S => {
                pos.y -= 1;
                pos.z += 1;
            }
            HexDir::SE => {
                pos.y -= 1;
                pos.x += 1;
            },
            HexDir::SW => {
                pos.x -= 1;
                pos.z += 1;
            }
        }

        furthest_distance = cmp::max(
            furthest_distance,
            pos.distance_from_center()
        );
    }

    let final_distance = pos.distance_from_center();
    println!("final distance is {}", final_distance);
    println!("furthest distance is {}", furthest_distance);
}

enum HexDir {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
}

#[derive(Debug)]
struct HexPos {
    x: i32,
    y: i32,
    z: i32,
}

impl HexPos {
    fn distance_from_center(&self) -> u32 {
        let result = cmp::max(self.x.abs(), self.y.abs());
        cmp::max(result, self.z.abs()) as u32
    }
}


