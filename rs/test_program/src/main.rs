fn main() {
    let test;
    {
        test = "12345".to_string();
    }

    println!("{}", test);

    // let true_false = get_true_false(20);
    // println!("{:?}", true_false);

    // let true_false = get_true_false(35);
    // println!("{:?}", true_false);
}

// fn get_true_false(vec_size: i32) -> Vec<&'static str> {
//     let strings = vec!["true".to_string(), "false".to_string()];

//     let mut string_references: Vec<&str> = Vec::<&str>::new();

//     for i in 0..vec_size {
//         if i % 2 == 0 {
//             string_references.push(&strings[0]);
//         }
//         else {
//             string_references.push(&strings[1]);
//         }
//     }

//     string_references
// }