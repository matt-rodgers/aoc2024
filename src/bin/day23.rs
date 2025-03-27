use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let input = include_str!("../../inputs/23.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    let pairs: Vec<(&str, &str)> = input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in pairs {
        let ca = connections.entry(a).or_insert(HashSet::new());
        ca.insert(b);

        let cb = connections.entry(b).or_insert(HashSet::new());
        cb.insert(a);
    }

    let mut sets_of_three: HashSet<Vec<&str>> = HashSet::new();
    for (key, val) in connections.iter() {
        for combo in val.iter().combinations(2) {
            let a = combo[0];
            let b = combo[1];
            if let Some(ca) = connections.get(a) {
                if ca.contains(b) {
                    let mut set = Vec::new();
                    set.push(*key);
                    set.push(a);
                    set.push(b);
                    set.sort(); // Must be sorted to avoid duplicates in HashSet
                    sets_of_three.insert(set);
                }
            }
        }
    }

    let pt1 = sets_of_three
        .iter()
        .filter(|s| s.iter().any(|elem| elem.starts_with('t')))
        .count();

    (pt1 as u64, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/23.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 7);
        assert_eq!(pt2, 0);
    }
}
