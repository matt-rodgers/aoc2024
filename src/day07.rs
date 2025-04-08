use std::time::Instant;

pub fn run_outer() -> String {
    let input = include_str!("../inputs/07.in");
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

#[derive(Debug)]
struct Equation {
    testval: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let cpos = s.find(':').unwrap();
        let testval: u64 = s[0..cpos].parse().unwrap();
        let numbers: Vec<u64> = s[(cpos + 1)..]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Equation { testval, numbers }
    }
}

/// Helper function for concatenate operator.
/// Concatenation can be done faster than converting to strings and back by observing that e.g.
///   123 || 45 == (123 * 100) + 45
/// So we just need to identify the first multiple of 10 larger than the second number, multiply
/// the first number by that, and then add them.
fn concatenate(a: u64, b: u64) -> u64 {
    let mut multiplier = 10;

    while b >= multiplier {
        multiplier = multiplier * 10;
    }

    (a * multiplier) + b
}

fn is_solvable_helper(target: u64, numbers: &[u64], total: u64, pt2: bool) -> bool {
    // Recursion finished, check if result matches test value
    if numbers.is_empty() {
        return total == target;
    }

    // Early abort if we exceed testval, since operators can only increase the total
    if total > target {
        return false;
    }

    // Recurse trying all possible operations
    is_solvable_helper(target, &numbers[1..], total + numbers[0], pt2)
        || is_solvable_helper(target, &numbers[1..], total * numbers[0], pt2)
        || (pt2 && is_solvable_helper(target, &numbers[1..], concatenate(total, numbers[0]), pt2))
}

impl Equation {
    fn is_solvable(&self, pt2: bool) -> bool {
        is_solvable_helper(self.testval, &self.numbers[1..], self.numbers[0], pt2)
    }
}

fn run(input: &str) -> (u64, u64) {
    let equations: Vec<Equation> = input
        .trim_end()
        .lines()
        .map(|line| Equation::from(line))
        .collect();

    let pt1: u64 = equations
        .iter()
        .filter_map(|equation| match equation.is_solvable(false) {
            true => Some(equation.testval),
            false => None,
        })
        .sum();

    let pt2: u64 = equations
        .iter()
        .filter_map(|equation| match equation.is_solvable(true) {
            true => Some(equation.testval),
            false => None,
        })
        .sum();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/07.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 3749);
        assert_eq!(pt2, 11387);
    }

    #[test]
    fn test_solvable_pt1() {
        let equation = Equation {
            testval: 190,
            numbers: vec![10, 19],
        };
        assert!(equation.is_solvable(false));

        let equation = Equation {
            testval: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(equation.is_solvable(false));

        let equation = Equation {
            testval: 292,
            numbers: vec![11, 6, 16, 20],
        };
        assert!(equation.is_solvable(false));

        let equation = Equation {
            testval: 161011,
            numbers: vec![16, 10, 13],
        };
        assert!(!equation.is_solvable(false));
    }
}
