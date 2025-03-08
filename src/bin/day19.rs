use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../inputs/19.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn pattern_count_combinations<'a>(
    pattern: &'a str,
    towels: &Vec<&str>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Recursion end condition
    if pattern.len() == 0 {
        return 1;
    }

    // Check for result in cache first
    if let Some(res) = cache.get(pattern) {
        return *res;
    }

    let mut combinations = 0;
    for towel in towels.iter() {
        if pattern.starts_with(towel) {
            let next_slice = &pattern[towel.len()..];
            combinations += pattern_count_combinations(next_slice, towels, cache);
        }
    }

    cache.insert(pattern, combinations);
    return combinations;
}

fn run(input: &str) -> (u64, u64) {
    let (towel_line, pattern_lines) = input.trim().split_once("\n\n").unwrap();

    let towels: Vec<&str> = towel_line.split(", ").collect();

    let mut cache = HashMap::<&str, u64>::new();

    let mut pt1 = 0;
    let pt2 = pattern_lines
        .lines()
        .map(|line| {
            let combinations = pattern_count_combinations(line, &towels, &mut cache);
            if combinations > 0 {
                pt1 += 1;
            }
            combinations
        })
        .sum();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/19.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 6);
        assert_eq!(pt2, 16);
    }
}
