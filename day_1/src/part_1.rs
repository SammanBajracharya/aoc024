use std::fs::read_to_string;
use crate::quicksort::quicksort;

pub fn part_1(file_path: String) -> i32 {
    let mut left_arr: Vec<i32> = vec![];
    let mut right_arr: Vec<i32> = vec![];
    let contents = read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let left_num = parts[0].parse::<i32>();
        let right_num = parts[1].parse::<i32>();
        if let (Ok(left), Ok(right)) = (left_num, right_num) {
            left_arr.push(left);
            right_arr.push(right);
        }
    }

    quicksort(&mut left_arr);
    quicksort(&mut right_arr);

    let mut sum: i32 = 0;
    for i in 0..left_arr.len() {
        sum += (left_arr[i] - right_arr[i]).abs();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_file_path = String::from("./test.txt");
        let result_1 = part_1(test_file_path);

        let input_file_path = String::from("./input.txt");
        let result_2 = part_1(input_file_path);

        assert_eq!(result_1, 11);
        assert_eq!(result_2, 1660292);
    }
}
