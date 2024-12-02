use std::fs::read_to_string;
use crate::quicksort::{quicksort, SortType};

pub fn part_1(file_path: String) -> i32 {
    let contents = read_to_string(file_path).expect("File not Found");
    let safe_report = contents.lines()
        .filter_map(|line| {
            parse_line(line).ok()
        })
        .filter(|report| is_safe_report(report))
        .count();

    safe_report as i32
}

fn parse_line(line: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    line.split_whitespace()
        .map(str::parse)
        .collect()
}

fn is_safe_report(report: &[i32]) -> bool {
    if !is_valid_difference(report) {
        return false;
    }

    match sort_report(report.to_vec()) {
        Ok(sorted_report) => report.to_vec() == sorted_report,
        Err(_) => false
    }
}

fn is_valid_difference(report: &[i32]) -> bool {
    report.windows(2)
        .all(|window| (window[0] - window[1]).abs() >= 1 && (window[0] - window[1]).abs() <= 3)
}

fn sort_report(mut report: Vec<i32>) -> Result<Vec<i32>, ()> {
    let order_indicator = report.windows(2)
        .find_map(|window| {
            let diff = window[0] - window[1];
            (diff != 0).then_some(diff)
        })
        .ok_or(())?;

    let sort_type = if order_indicator > 0 { SortType::Desc } else { SortType::Asc };

    quicksort(&mut report, sort_type);

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_part_1 = part_1(String::from("./test.txt"));
        let input_part_1 = part_1(String::from("./input.txt"));

        assert_eq!(test_part_1, 2);
        assert_eq!(input_part_1, 606);
    }
}
