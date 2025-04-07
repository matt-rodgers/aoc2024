use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/05.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed)
}

/// Return true if the update is valid
fn update_is_valid(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, val) in update.iter().enumerate() {
        match rules.get(&val) {
            None => {
                // No rules relevant to this value, so update is still valid
            }
            Some(hs) => {
                // There are rules relevant to this value. A rule is violated if any number in the
                // HashSet occurs before this number in the sequence.
                for other in update[..i].iter() {
                    if hs.contains(other) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn rearrange_update(
    update: &[u32],
    rules: &HashMap<u32, HashSet<u32>>,
    start_from: usize,
) -> Vec<u32> {
    let mut new_update = Vec::from(update);
    let mut i: usize = start_from;

    while i < update.len() {
        // For each rerun_outer() -> Stringing item in the slice, first get any associated rules
        let hs = match rules.get(&new_update[i]) {
            None => {
                i += 1;
                continue;
            }
            Some(hs) => hs,
        };

        // Then collect all the indices of values that break the rules
        let mut indices_to_move: Vec<usize> = new_update[..i]
            .iter()
            .enumerate()
            .filter_map(|(n, val)| match hs.contains(val) {
                true => Some(n),
                false => None,
            })
            .collect();

        // For each value that requires moving, remove it from the vec and then re-insert after i,
        // remembering to update i to account for the removal. We are specifically using pop here
        // to remove elements from the end of the vector. This means that we don't need to update
        // the indices of items still to be removed, which would be a pain.
        while let Some(n) = indices_to_move.pop() {
            let swap = new_update.remove(n);
            new_update.insert(i, swap);
            i -= 1;
        }

        i += 1;
    }

    // It's quite quick to validate the update, so may as well confirm before returning it
    assert!(update_is_valid(&new_update, rules));
    new_update
}

fn get_rules_and_updates(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let (rules, updates) = input.trim().split_at(input.find("\n\n").unwrap());

    let rules: Vec<(u32, u32)> = rules
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .trim()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    // Store the rules more efficiently as a map of X: Vec<Y>
    let mut rules_hm: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (x, y) in rules {
        match rules_hm.get_mut(&x) {
            None => {
                let mut hs = HashSet::new();
                hs.insert(y);
                rules_hm.insert(x, hs);
            }
            Some(hs) => {
                hs.insert(y);
            }
        };
    }

    (rules_hm, updates)
}

fn run(input: &str) -> (u64, u64) {
    let (rules, updates) = get_rules_and_updates(input);

    let pt1: u64 = updates
        .iter()
        .map(|update| {
            match update_is_valid(&update, &rules) {
                true => {
                    assert!(update.len() % 2 == 1); // Update must have an odd number of elements
                    update[(update.len() - 1) / 2] as u64
                }
                false => 0,
            }
        })
        .sum();

    let pt2: u64 = updates
        .iter()
        .map(|update| {
            match update_is_valid(&update, &rules) {
                true => 0,
                false => {
                    let new_update = rearrange_update(&update, &rules, 0);
                    assert!(new_update.len() % 2 == 1); // Update must have an odd number of elements
                    new_update[(new_update.len() - 1) / 2] as u64
                }
            }
        })
        .sum();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/05.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 143);
        assert_eq!(pt2, 123);
    }

    #[test]
    fn test_update_is_valid() {
        let input = include_str!("../inputs/05.ex");
        let (rules, _updates) = get_rules_and_updates(input);

        // This update is valid
        let update = vec![75, 47, 61, 53, 29];
        let valid = update_is_valid(&update, &rules);
        assert!(valid);

        // This update is not valid due to 75 being before 97
        let update = vec![75, 97, 47, 61, 53];
        let valid = update_is_valid(&update, &rules);
        assert!(!valid);
    }

    #[test]
    fn test_rearrange_update() {
        let input = include_str!("../inputs/05.ex");
        let (rules, _updates) = get_rules_and_updates(input);

        let update = vec![75, 97, 47, 61, 53];
        let new_update = rearrange_update(&update, &rules, 0);
        assert_eq!(new_update, vec![97, 75, 47, 61, 53]);

        let update = vec![61, 13, 29];
        let new_update = rearrange_update(&update, &rules, 0);
        assert_eq!(new_update, vec![61, 29, 13]);
    }
}
