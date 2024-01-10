use rand::Rng;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let grid_length = 30;

    let mut grid: Vec<bool> = vec![];
    for _ in 0..grid_length {
        grid.push(rng.gen());
    }
    let mut new_grid: Vec<bool> = vec![];

    loop {
        new_grid.clear();
        for i in 0..grid.len() {
            new_grid.push(!(&grid[i]) || rng.gen_range(0..100) > 90);
        }

        println!(
            "{}",
            grid.iter()
                .map(|&x| {
                    if x {
                        1.to_string()
                    } else {
                        0.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        );

        for i in 0..new_grid.len() {
            grid[i] = new_grid[i];
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
