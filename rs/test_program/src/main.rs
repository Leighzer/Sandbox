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

// used this function to help wrap my mind around references and lifetimes
// fn get_true_false<'a>(vec_size: i32) -> Vec<&'a str> {
//     let strings = vec!["true", "false"];

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