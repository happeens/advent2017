use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = File::open("input.txt").expect("input not found");
    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .expect("could not read input to string");

    let firewall = contents.split("\n")
        .filter(|it| !it.is_empty())
        .map(|it| Layer::from_string(it))
        .collect::<Firewall>();

    let severity = calc_severity(&firewall);
    println!("severity is {}", severity);

    let mut delay = 0;
    while has_hits(&firewall, delay) {
        delay += 1;
    }

    println!("delay needed: {}", delay);
}

fn has_hits(firewall: &Firewall, delay: u32) -> bool {
    firewall.iter().any(|layer| {
        let mod_range = (2 * layer.range)
            .checked_sub(2)
            .unwrap_or(0);

        (layer.depth + delay) % mod_range == 0
    })
}

fn calc_severity(firewall: &Firewall) -> u32 {
    let mut firewall = firewall.clone();
    let layer_count = firewall.iter()
        .max_by_key(|it| it.depth)
        .unwrap().depth;

    let mut self_pos: Position = 0;
    let mut severity = 0;

    loop {
        let found = firewall.iter().cloned()
            .find(|it| it.depth == self_pos);

        if let Some(layer) = found {
            if layer.scanner_pos == 0 {
                severity += layer.range * layer.depth;
            }
        }

        firewall.move_scanners();

        self_pos += 1;
        if self_pos > layer_count { break; }
    }

    severity
}

type Position = u32;
type Firewall = Vec<Layer>;
trait FirewallMethods {
    fn move_scanners(&mut self);
}

impl FirewallMethods for Firewall {
    fn move_scanners(&mut self) {
        for it in self.iter_mut() {
            it.move_scanner();
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Layer {
    depth: u32,
    range: u32,
    scanner_pos: Position,
    scanner_dir: Direction,
}

impl Layer {
    fn from_string(from: &str) -> Layer {
        let parts = from.split(":")
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
            .collect::<Vec<&str>>();

        assert!(parts.len() == 2);

        let depth = String::from(parts[0])
            .parse::<u32>().unwrap();
        let range = String::from(parts[1])
            .parse::<u32>().unwrap();

        Layer {
            depth,
            range,
            scanner_pos: 0,
            scanner_dir: Direction::Down,
        }
    }

    fn move_scanner(&mut self) {
        match self.scanner_dir {
            Direction::Down => {
                self.scanner_pos += 1;
                if self.scanner_pos == self.range - 1 {
                    self.scanner_dir = Direction::Up;
                }
            },
            Direction::Up => {
                self.scanner_pos -= 1;
                if self.scanner_pos == 0 {
                    self.scanner_dir = Direction::Down;
                }
            },
        }
    }
}
