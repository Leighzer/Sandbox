use std::env;
use std::time::Duration;
use text_colorizer::*;

// TODO cleanup reduce reversing calls in this program but code does work for now

fn main() {
    let grid_length = 41;

    // Parse the command line argument to a u8
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rule_number>", args[0]);
        std::process::exit(1);
    }
    let num = args[1].parse::<u8>().expect("Error parsing ruleset arg");

    let mut rule_set: Vec<bool> = u8_to_vec_bool(num);
    rule_set.reverse();

    println!("{:?}", rule_set);

    let mut grid: Vec<bool> = vec![];
    for i in 0..grid_length {
        if i == 21 {
            grid.push(true);
        } else {
            grid.push(false);
        }
    }
    let mut new_grid: Vec<bool> = vec![];

    loop {
        new_grid.clear();

        for i in 0..grid.len() {
            let start = match (i as i32) - 1 {
                -1 => grid.len() - 1,
                result => result as usize,
            };
            let mid = i;
            let end = (i + 1) % grid.len();

            let mut index = 0;
            if grid[start] {
                index += 1;
            }
            if grid[mid] {
                index += 2;
            }
            if grid[end] {
                index += 4;
            }
            new_grid.push(rule_set[index]);
        }

        println!(
            "{}",
            grid.iter()
                .map(|&x| {
                    if x {
                        1.to_string().red().bold().to_string()
                    } else {
                        0.to_string().blue().to_string()
                    }
                })
                .rev()
                .collect::<Vec<String>>()
                .join(" ")
        );

        // for i in 0..new_grid.len() {
        //     grid[i] = new_grid[i];
        // }
        // recommended per cargo clippy
        grid[..new_grid.len()].copy_from_slice(&new_grid[..]);
        std::thread::sleep(Duration::from_millis(200));
    }
}

fn u8_to_vec_bool(num: u8) -> Vec<bool> {
    let mut bits = vec![];
    for i in (0..8).rev() {
        let bit_mask = 1_u8 << i;
        let ith_bit = num & bit_mask != 0;
        bits.push(ith_bit);
    }
    bits
}
