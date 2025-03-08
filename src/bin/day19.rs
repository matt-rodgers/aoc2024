use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../inputs/19.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn pattern_is_possible<'a>(
    pattern: &'a str,
    towels: &Vec<&str>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    // Recursion end condition
    if pattern.len() == 0 {
        return true;
    }

    // Check for result in cache first
    if let Some(res) = cache.get(pattern) {
        return *res;
    }

    for towel in towels.iter() {
        if pattern.starts_with(towel) {
            assert!(towel.len() > 0);
            let next_slice = &pattern[towel.len()..];

            if pattern_is_possible(next_slice, towels, cache) {
                cache.insert(pattern, true);
                return true;
            }
        }
    }

    cache.insert(pattern, false);
    return false;
}

fn run(input: &str) -> (u64, u64) {
    let (towel_line, pattern_lines) = input.trim().split_once("\n\n").unwrap();

    let towels: Vec<&str> = towel_line.split(", ").collect();

    let mut cache = HashMap::<&str, bool>::new();

    let pt1 = pattern_lines
        .lines()
        .map(|line| pattern_is_possible(line, &towels, &mut cache))
        .filter(|b| *b)
        .count();

    (pt1 as u64, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/19.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 6);
        assert_eq!(pt2, 0);
    }
}
