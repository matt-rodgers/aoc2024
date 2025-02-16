use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/DAY.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/DAY.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 0);
        assert_eq!(pt2, 0);
    }
}
