// naive solution to Maximum Subarray Problem
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let numbers: Vec<i32> = args.into_iter().map(|arg| -> i32 {
        arg.parse().unwrap()
    }).collect();
    
    println!("{:?}", max_subarray_naive(&numbers));
}

fn max_subarray_kadane(numbers: &Vec<i32>) -> Vec<i32> {
    let mut max_subarray_sum = i32::MIN;
    let mut current_subarray_sum = 0;
    let mut start = 0;
    let mut end = 0;
    let mut s = 0;

    for i in 0..(numbers.len()) {
        current_subarray_sum += numbers[i];
        
        if current_subarray_sum > max_subarray_sum {
            max_subarray_sum = current_subarray_sum;
            start = s;
            end = i;
        }

        if current_subarray_sum < 0 {
            current_subarray_sum = 0;
            s = i + 1;
        }
    }
    
    numbers[start..=end].to_vec()
}

fn max_subarray_naive(numbers: &Vec<i32>) -> Vec<i32> {
    let mut max_subarray_sum: i32 = i32::MIN;
    let mut max_subarray: Vec<i32> = Vec::<i32>::new();
    
    for i in 1..=(numbers.len()) { // loop for different sub-array sizes
        for j in 0..=(numbers.len() - i){ // loop for scanning our sub-array over entire array
            let start = j;
            let end = j + i - 1;

            //println!("i:{} j:{} start:{} end:{}",i, j, start, end);

            let subarray = &numbers[start..=end];
            let mut subarray_sum = 0;
            for k in subarray {
                subarray_sum += k;
            }

            if subarray_sum >= max_subarray_sum {
                max_subarray_sum = subarray_sum;
                max_subarray = subarray.to_vec();
            }
        }
    }

    max_subarray
}

#[test]
fn test_max_subarray_naive_sum() {
    assert_eq!(max_subarray_naive(&vec![-2, -3, 4, -1, -2, 1, 5, -3]), vec![4, -1, -2, 1, 5]);
    assert_eq!(max_subarray_naive(&vec![-1, -2, 3, 5, -1, 2, -4, 2, -6]), vec![3, 5, -1, 2]);
    assert_eq!(max_subarray_naive(&vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), vec![4, -1, 2, 1]);
    assert_eq!(max_subarray_naive(&vec![8, -19, 5, -4, 20]), vec![5, -4, 20]);
}

#[test]
fn test_max_subarray_hopefully_better_sum() {
    assert_eq!(max_subarray_kadane(&vec![-2, -3, 4, -1, -2, 1, 5, -3]), vec![4, -1, -2, 1, 5]);
    assert_eq!(max_subarray_kadane(&vec![-1, -2, 3, 5, -1, 2, -4, 2, -6]), vec![3, 5, -1, 2]);
    assert_eq!(max_subarray_kadane(&vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), vec![4, -1, 2, 1]);
    assert_eq!(max_subarray_kadane(&vec![8, -19, 5, -4, 20]), vec![5, -4, 20]);
    assert_eq!(max_subarray_kadane(&vec![-8, -19, -5, -4, -20]), vec![-4]);
    assert_eq!(max_subarray_kadane(&vec![500, -19, -5, -4, -20]), vec![500]);
    assert_eq!(max_subarray_kadane(&vec![18, -19, -5, -4, -20]), vec![18]);
    assert_eq!(max_subarray_kadane(&vec![-4, 5, -4]), vec![5]);
}