use std::fs::read_to_string;

enum ConditionState {
    Do,
    Not,
}

pub fn part_2(file_path: String) -> i32 {
    let contents = read_to_string(file_path).unwrap();
    let mut result: i32 = 0;
    let chars: Vec<char> = contents.chars().collect();
    let mut index = 0;

    let mut state: ConditionState = ConditionState::Do;

    while index < chars.len() {
        if chars[index] == 'd' {
            if &contents[index..=index+6] == "don't()" {
                state = ConditionState::Not;
            } else if &contents[index..=index+3] == "do()" {
                state = ConditionState::Do;
            }
        }

        match state {
            ConditionState::Do => {
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
            }
            ConditionState::Not => {}
        }
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let result_test_data = part_2(String::from("./test.txt"));
        let result_input_data = part_2(String::from("./input.txt"));

        assert_eq!(result_test_data, 48);
        assert_eq!(result_input_data, 77055967);
    }
}
