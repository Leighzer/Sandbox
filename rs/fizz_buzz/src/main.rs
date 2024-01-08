use std::env;

fn main() {
    let args = parse_args();

    let fizz_buzz = fizz_buzz(args.input_integer);

    println!("{:?}", fizz_buzz);
}

fn fizz_buzz(input_integer: i32) -> Vec<String> {
    let mut fizz_buzz = Vec::<String>::new();

    for i in 1..=input_integer {
        let is_divisible_by_3 = i % 3 == 0;
        let is_divisible_by_5 = i % 5 == 0;

        if is_divisible_by_3 && is_divisible_by_5 {
            fizz_buzz.push("FizzBuzz".to_string());
        }
        else if is_divisible_by_3 {
            fizz_buzz.push("Fizz".to_string());
        }
        else if is_divisible_by_5 {
            fizz_buzz.push("Buzz".to_string());
        }
        else {
            fizz_buzz.push(i.to_string());
        }
    }

    fizz_buzz
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 1, got {}", "Error:", args.len());
        std::process::exit(1);
    }

    let input_integer = match args[0].parse::<i32>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read int argument '{}': {:?}", "Error:", args[0], e);
            std::process::exit(1);
        } 
    };

    if input_integer < 1 || input_integer > i32::MAX {
        eprintln!("{} argument '{}' is not within the range of valid values {} - {}", "Error:", input_integer, 1, i32::MAX);
        std::process::exit(1);
    }

    Arguments{
        input_integer: input_integer
    }
}

fn print_usage() {
    eprintln!("{} - provide an integer to get a list to get a list of strings following the classic rules of FizzBuzz", "fizz_buzz");
    eprintln!("Usage: fizz_buzz <input_integer>");
}

#[derive(Debug)]
struct Arguments {
    input_integer: i32,
}

#[test]
fn test_max_subarray_naive_sum() {
    assert_eq!(fizz_buzz(1), vec!["1"]);
    assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(fizz_buzz(5), vec!["1","2","Fizz","4","Buzz"]);
    assert_eq!(fizz_buzz(15), vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]);
}