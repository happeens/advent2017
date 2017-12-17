extern crate advent2017;
use advent2017::knot;

use std::u16;
use std::collections::HashSet;

const GRID_SIZE: usize = 128;

fn main() {
    let input = String::from("ugkiagan");

    let binary_rows = (0..GRID_SIZE)
        .map(|it| format!("{}-{}", input, it))
        .map(|it| knot::hash_str(&it))
        .map(|it| it.chars()
            .map(|it| u16::from_str_radix(&it.to_string(), 16).unwrap())
            .map(|it| format!("{:04b}", it))
            .collect::<Vec<String>>()
            .join("")
        )
        .collect::<Vec<String>>();

    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] =
        [[false; GRID_SIZE]; GRID_SIZE];
    let mut active_squares = 0;

    for (y, row) in binary_rows.iter().enumerate() {
        for (x, it) in row.chars().enumerate() {
            if x >= GRID_SIZE { break; }
            if it == '1' {
                grid[y][x] = true;
                active_squares += 1;
            }
        }
    }

    println!("active squares: {}", active_squares);

    let mut visited = HashSet::new();
    let mut regions = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited.contains(&(x, y)) { continue; }
            if grid[y][x] {
                mark_region((x, y), &mut visited, grid);
                regions += 1;
            }
        }
    }

    println!("total region count: {}", regions);
}

fn mark_region(
    pos: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    grid: [[bool; GRID_SIZE]; GRID_SIZE]
) {
    let mut to_check = vec![pos];
    while let Some(it) = to_check.pop() {
        if visited.contains(&it) { continue; }
        let (x, y) = it;
        if !grid[y][x] { continue; }

        visited.insert(it);

        if x > 0 { to_check.push((x - 1, y)) }
        if x < GRID_SIZE - 1 { to_check.push((x + 1, y)) }
        if y > 0 { to_check.push((x, y - 1)) }
        if y < GRID_SIZE - 1 { to_check.push((x, y + 1)) }
    }
}
