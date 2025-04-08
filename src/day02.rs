use std::time::Instant;

pub fn run_outer() -> String {
    let input = include_str!("../inputs/02.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!(
        "pt1: {} , pt2: {} , elapsed time {:?} us",
        pt1,
        pt2,
        elapsed.as_micros()
    )
}

fn check_safety(numbers: &[i64]) -> bool {
    let diffs: Vec<i64> = numbers.windows(2).map(|sl| sl[0] - sl[1]).collect();

    let all_increasing = diffs.iter().all(|diff| *diff > 0);
    let all_decreasing = diffs.iter().all(|diff| *diff < 0);

    if !all_decreasing && !all_increasing {
        return false;
    }

    let maxdiff: i64 = diffs.iter().map(|diff| diff.abs()).max().unwrap();

    if maxdiff > 3 {
        return false;
    }

    true
}

fn report_to_numbers(report: &str) -> Vec<i64> {
    report
        .split_whitespace()
        .map(|n| i64::from_str_radix(n, 10).unwrap())
        .collect()
}

fn report_is_safe_pt1(report: &str) -> bool {
    let numbers = report_to_numbers(report);
    check_safety(&numbers)
}

fn report_is_safe_pt2(report: &str) -> bool {
    let numbers = report_to_numbers(report);

    (0..numbers.len())
        .map(|i| {
            // For each index i in numbers, create a new vec with that index removed
            let numbers_with_skip: Vec<i64> = numbers
                .iter()
                .enumerate()
                .filter_map(|(j, n)| if i == j { None } else { Some(*n) })
                .collect();

            // Check if the report would be safe with the index i removed
            check_safety(&numbers_with_skip)
        })
        .any(|val| val) // The overall report is safe if any one of the reports with an index skipped is safe
}

fn run(input: &str) -> (u64, u64) {
    let reports: Vec<&str> = input.trim_end().lines().collect();

    let pt1: u64 = reports
        .iter()
        .filter(|report| report_is_safe_pt1(report))
        .count() as u64;

    let pt2: u64 = reports
        .iter()
        .filter(|report| report_is_safe_pt2(report))
        .count() as u64;

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/02.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 2);
        assert_eq!(pt2, 4);
    }
}
