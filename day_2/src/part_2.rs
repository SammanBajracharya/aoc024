use std::fs::read_to_string;

pub fn part_2(file_path: String) -> i32 {
    let mut safe_report = 0;
    let contents = read_to_string(file_path).expect("File Not Found");

    for line in contents.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .map(|res| match res {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("Failed to parse: {}", e);
                    0
                }
            })
            .collect();
        if is_valid(&report) {
            println!("Safe Report: {:?}", report);
            safe_report += 1
        }
    }

    safe_report
}

fn is_valid(arr: &[i32]) -> bool {
    if is_safe(arr) { return true; }

    for i in 0..arr.len() {
        let mut modified_arr = arr.to_vec();
        modified_arr.remove(i);

        if is_safe(&modified_arr) { return true; }
    }

    false
}

fn is_safe(arr: &[i32]) -> bool {
    if arr.len() < 2 { return true; }

    let differences: Vec<i32> = arr.windows(2)
        .map(|w| w[0] - w[1])
        .collect();

    if !differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3) { return false; }

    let all_increasing = differences.iter().all(|&diff| diff < 0);
    let all_decreasing = differences.iter().all(|&diff| diff > 0);

    all_increasing || all_decreasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let test_part_2 = part_2(String::from("./test.txt"));
        let input_part_2 = part_2(String::from("./input.txt"));

        assert_eq!(test_part_2, 4);
        assert_eq!(input_part_2, 644);
    }
}
