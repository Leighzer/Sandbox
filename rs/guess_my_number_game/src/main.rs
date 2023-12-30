use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() {
    let mut is_done: bool = false;
    
    let mut rng = rand::thread_rng();

    let min: u8 = 0;
    let max: u8 = 10;

    let random_val = rng.gen_range(min..=max);
    
    println!("I have generated a random number between {0} - {1}.", min, max);

    while !is_done {
        println!("Take a guess!");
        let mut input = String::new();
        let mut is_input_good = false;
        while !is_input_good {
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            match u8::from_str(input.trim()) {
                Ok(num) => {
                    println!("You entered: {0}.", num);
    
                    if num < random_val {
                        println!("Guess higher.");
                    }
                    else if num == random_val {
                        println!("You got it right!");
                        is_done = true;
                    }
                    else {
                        println!("Guess lower.");
                    }
                    is_input_good = true;
                }
                Err(_) => {
                    println!("Please enter a valid number!");
                }
            }
            // clear bad input
            input = String::new();
        }
    }

    println!("Congrats you have won! Goodbye.");
}
