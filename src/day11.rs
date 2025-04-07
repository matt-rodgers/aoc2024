use std::{collections::HashMap, time::Instant};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/11.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed)
}

fn run(input: &str) -> (u64, u64) {
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cache = HashMap::<(u64, u32), u64>::new();

    let pt1: u64 = stones
        .iter()
        .map(|stone| count_stones_recursive(*stone, 25, &mut cache))
        .sum();

    let pt2: u64 = stones
        .iter()
        .map(|stone| count_stones_recursive(*stone, 75, &mut cache))
        .sum();

    (pt1, pt2)
}

/// Return the number of stones produced by this stone after the given number of steps
fn count_stones_recursive(
    stone: u64,
    steps_remaining: u32,
    cache: &mut HashMap<(u64, u32), u64>,
) -> u64 {
    // Check end condition of recursion
    if steps_remaining == 0 {
        return 1;
    }

    // Return result from cache if available
    if let Some(res) = cache.get(&(stone, steps_remaining)) {
        return *res;
    }

    let next_steps = steps_remaining - 1;

    let res = if stone == 0 {
        count_stones_recursive(1, next_steps, cache)
    } else {
        let digits = count_digits(stone);

        if digits % 2 == 0 {
            let (left, right) = split_digits_at(stone, digits / 2);
            count_stones_recursive(left, next_steps, cache)
                + count_stones_recursive(right, next_steps, cache)
        } else {
            count_stones_recursive(stone * 2024, next_steps, cache)
        }
    };

    cache.insert((stone, steps_remaining), res);
    res
}

/// Count the number of digits in the base 10 representation of a number
fn count_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

/// Split a number in half by digits, ie. 1234 would become 12, 34
fn split_digits_at(n: u64, s: u32) -> (u64, u64) {
    let mut d = 1;
    let mut s = s;
    while s > 0 {
        d *= 10;
        s -= 1;
    }

    let left = n / d;
    let right = n - left * d;

    (left, right)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/11.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 55312);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(9), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(34), 2);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(100), 3);
        assert_eq!(count_digits(529), 3);
        assert_eq!(count_digits(7839), 4);
        assert_eq!(count_digits(63245), 5);
        assert_eq!(count_digits(36923492837983279), 17);
    }

    #[test]
    fn test_split_at() {
        assert_eq!(split_digits_at(1234, 2), (12, 34));
        assert_eq!(split_digits_at(100000, 3), (100, 0));
        assert_eq!(split_digits_at(1234504567, 5), (12345, 4567));
    }
}
