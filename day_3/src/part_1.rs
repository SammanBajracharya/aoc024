use std::fs::read_to_string;

pub fn part_1(file_path: String) -> i32 {
    let contents = read_to_string(file_path).unwrap();
    let mut result: i32 = 0;
    let chars: Vec<char> = contents.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if index >= 3 && chars[index] == '(' && &contents[index-3..index] == "mul" {
            if let Some(end) = (index..=(index+8).min(contents.len()))
                .find(|&i| chars[i] == ')')
            {
                let inner = &contents[index+1..end];
                if inner.contains(',') {
                    let mul: i32 = contents[index+1..end]
                        .split(',')
                        .filter_map(|s| s.trim().parse::<i32>().ok())
                        .product();
                    result += mul;
                }
            }
        }
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result_test_data = part_1(String::from("./test.txt"));
        let result_input_data = part_1(String::from("./input.txt"));

        assert_eq!(result_test_data, 161);
        assert_eq!(result_input_data, 153469856);
    }
}
