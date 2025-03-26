use std::time::Instant;

const PRUNE_MOD: u64 = 16777216;

fn main() {
    let input = include_str!("../../inputs/22.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    let numbers: Vec<u64> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let pt1 = numbers.iter().map(|n| evolve(*n, 2000)).sum();

    (pt1, 0)
}

fn evolve(n: u64, cycles: u64) -> u64 {
    let mut val = n;

    for _ in 0..cycles {
        val = val ^ (val << 6);
        val = val.rem_euclid(PRUNE_MOD);
        val = val ^ (val >> 5);
        val = val.rem_euclid(PRUNE_MOD);
        val = val ^ (val << 11);
        val = val.rem_euclid(PRUNE_MOD);
    }

    val
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/22.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 37327623);
        assert_eq!(pt2, 0);
    }
}
