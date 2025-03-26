use std::{collections::HashMap, time::Instant};

const PRUNE_MOD: u32 = 16777216;

fn main() {
    let input = include_str!("../../inputs/22.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    let numbers: Vec<u32> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let sequences: Vec<Vec<u32>> = numbers.iter().map(|n| evolve(*n, 2000)).collect();

    let pt1 = sequences
        .iter()
        .map(|seq| *seq.last().unwrap() as u64)
        .sum();

    let mut overall_hm: HashMap<Vec<i32>, u32> = HashMap::new();

    for seq in sequences {
        let mut hm: HashMap<Vec<i32>, u32> = HashMap::new();

        for w in seq.windows(5) {
            let price = w[4] % 10;
            let diffs: Vec<i32> = w
                .windows(2)
                .map(|a| (a[1] % 10) as i32 - (a[0] % 10) as i32)
                .collect();

            // Only the first occurrence of any sequence is relevant
            if !hm.contains_key(&diffs) {
                hm.insert(diffs.clone(), price);

                // Update overall hashmap
                let total_price = overall_hm.entry(diffs).or_insert(0);
                *total_price += price;
            }
        }
    }

    let pt2 = overall_hm.values().max().unwrap();

    (pt1, *pt2 as u64)
}

fn evolve(n: u32, cycles: u32) -> Vec<u32> {
    let mut val = n;
    let mut res = Vec::with_capacity(cycles as usize);

    for _ in 0..cycles {
        val = val ^ (val << 6);
        val = val.rem_euclid(PRUNE_MOD);
        val = val ^ (val >> 5);
        val = val.rem_euclid(PRUNE_MOD);
        val = val ^ (val << 11);
        val = val.rem_euclid(PRUNE_MOD);
        res.push(val);
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/22.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 37327623);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../../inputs/22_2.ex");
        let (_pt1, pt2) = run(&input);
        assert_eq!(pt2, 23);
    }
}
