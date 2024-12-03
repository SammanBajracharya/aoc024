mod part_1;
mod part_2;

// use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    let result = part_2(String::from("./test.txt"));
    println!("Uncorrupted Result: {result}");
}
