use itertools::Itertools;
use num::integer::gcd;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let input = include_str!("../../inputs/08.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn is_in_arena(pt: (isize, isize), max: (isize, isize)) -> bool {
    pt.0 >= 0 && pt.1 >= 0 && pt.0 < max.0 && pt.1 < max.1
}

fn find_antinodes(
    a: (isize, isize),
    b: (isize, isize),
    max: (isize, isize),
    pt2: bool,
) -> Vec<(isize, isize)> {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let mut results = Vec::new();

    if !pt2 {
        let out1 = (a.0 - dx, a.1 - dy);
        let out2 = (b.0 + dx, b.1 + dy);
        if is_in_arena(out1, max) {
            results.push(out1);
        }
        if is_in_arena(out2, max) {
            results.push(out2);
        }
    } else {
        let d = gcd(dx, dy);
        let dx = dx / d;
        let dy = dy / d;

        // Start from only one of the nodes, move in each direction until past edge of arena
        let mut an = a;

        while is_in_arena(an, max) {
            results.push(an);
            an = (an.0 - dx, an.1 - dy);
        }

        an = (a.0 + dx, a.1 + dy);
        while is_in_arena(an, max) {
            results.push(an);
            an = (an.0 + dx, an.1 + dy);
        }
    }

    results
}

fn run(input: &str) -> (u64, u64) {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut xlen: isize = 0;
    let mut ylen: isize = 0;

    for (y, line) in input.trim().lines().enumerate() {
        ylen += 1;
        let mut xl: isize = 0;
        for (x, c) in line.chars().enumerate() {
            xl += 1;
            if c != '.' {
                if let Some(v) = antennas.get_mut(&c) {
                    v.push((x as isize, y as isize));
                } else {
                    antennas.insert(c, vec![(x as isize, y as isize)]);
                }
            }
        }
        xlen = xl;
    }

    let mut antinodes_pt1: HashSet<(isize, isize)> = HashSet::new();
    let mut antinodes_pt2: HashSet<(isize, isize)> = HashSet::new();

    for v in antennas.values() {
        for comb in v.iter().combinations(2) {
            let [a, b]: [_; 2] = comb.try_into().unwrap();
            for an in find_antinodes(*a, *b, (xlen, ylen), false) {
                antinodes_pt1.insert(an);
            }
            for an in find_antinodes(*a, *b, (xlen, ylen), true) {
                antinodes_pt2.insert(an);
            }
        }
    }

    (antinodes_pt1.len() as u64, antinodes_pt2.len() as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/08.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 14);
        assert_eq!(pt2, 34);
    }

    fn check_result(actual: Vec<(isize, isize)>, expected: Vec<(isize, isize)>) {
        let mut a = actual.clone();
        a.sort();
        let mut e = expected.clone();
        e.sort();
        assert_eq!(a, e);
    }

    #[test]
    fn test_antinodes() {
        let a = (4, 3);
        let b = (5, 5);
        let out = find_antinodes(a, b, (10, 10), false);
        check_result(out, vec![(3, 1), (6, 7)]);

        let a = (8, 1);
        let b = (5, 2);
        let out = find_antinodes(a, b, (12, 12), false);
        check_result(out, vec![(11, 0), (2, 3)]);
    }

    #[test]
    fn test_antinodes_pt2() {
        let a = (0, 0);
        let b = (1, 2);
        let out = find_antinodes(a, b, (10, 10), true);
        check_result(out, vec![a, b, (2, 4), (3, 6), (4, 8)]);

        let a = (0, 0);
        let b = (3, 1);
        let out = find_antinodes(a, b, (10, 10), true);
        check_result(out, vec![a, b, (6, 2), (9, 3)]);
    }
}
