use std::collections::HashMap;

fn main() {
    // sidelength increases by 2 each layer
    let mut sidelength = 1;
    let mut current_max = 1;
    let mut current_layer = 0;
    let mut next_min = 1;

    let input = 368078;

    loop {
        sidelength += 2;
        current_layer += 1;

        let current_min = next_min;
        let layer_count = (sidelength - 1) * 4;
        current_max += layer_count;
        next_min = current_max + 1;

        if input <= current_max && input >= current_min {
            let offset: i32 = ((input - current_min) % (sidelength - 1)) - (current_layer - 1);
            let offset = offset.abs();
            let steps = offset + current_layer;
            println!("{} needs {} steps", input, steps);
            break;
        }
    }

    let mut fields = HashMap::new();
    fields.insert((0i32, 0i32), 1u32);
    let mut sidelength = 3;
    let (mut x, mut y) = (0, 0);
    let directions = vec![
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 0)
    ];

    'outer: loop {
        x += 1;

        for dir in &directions {
            for i in 0..sidelength - 1 {
                // if starting new layer, stay in place for 1 round
                if !(i == 0 && *dir == (0, 1)) {
                    x += dir.0;
                    y += dir.1;
                }

                let mut acc = 0;
                for y_offset in -1..2 {
                    for x_offset in -1..2 {
                        if (x_offset, y_offset) == (0, 0) { continue; }

                        acc += fields.get(&(x + x_offset, y + y_offset)).unwrap_or(&0);
                    }
                }

                if acc > input as u32 {
                    println!("the first value written bigger than {} is {}", input, acc);
                    break 'outer;
                }

                fields.insert((x, y), acc);
            }
        }

        sidelength += 2;
    }
}
