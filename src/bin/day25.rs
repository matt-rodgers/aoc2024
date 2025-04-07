use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/25.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Clone)]
struct Schematic {
    heights: Vec<u32>,
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut heights = vec![0; 5];

        for line in value.trim().lines().skip(1).take(5) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[i] += 1;
                }
            }
        }

        Schematic { heights }
    }
}

fn run(input: &str) -> (u64, u64) {
    let mut keys: Vec<Schematic> = Vec::new();
    let mut locks: Vec<Schematic> = Vec::new();

    for block in input.trim().split("\n\n") {
        let sch = Schematic::from(block);
        if block.starts_with('.') {
            keys.push(sch);
        } else {
            locks.push(sch);
        }
    }

    let mut pt1 = 0;
    'outer: for (k, l) in keys.iter().cartesian_product(locks) {
        for i in 0..5 {
            if k.heights[i] + l.heights[i] > 5 {
                continue 'outer;
            }
        }

        pt1 += 1;
    }

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/25.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 3);
    }
}
