use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/01.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    let num_iter = input
        .split_whitespace()
        .map(|n| i64::from_str_radix(n, 10).unwrap());

    let mut left: Vec<i64> = num_iter.clone().step_by(2).collect();
    let mut right: Vec<i64> = num_iter.skip(1).step_by(2).collect();

    left.sort();
    right.sort();

    let pt1: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs() as u64)
        .sum();

    let pt2: u64 = left
        .iter()
        .map(|l| {
            let count = right.iter().filter(|r| *r == l).count();
            *l as u64 * count as u64
        })
        .sum();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/01.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 11);
        assert_eq!(pt2, 31);
    }
}
