use crate::neighbor;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet, time::Instant};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/18.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed)
}

fn run(input: &str) -> (u64, String) {
    run_inner(input, 70, 1024)
}

fn shortest_path(
    all_positions: &Vec<(usize, usize)>,
    max_dim: usize,
    sim_limit: usize,
) -> Option<u64> {
    let positions: HashSet<(usize, usize)> =
        all_positions.iter().take(sim_limit).cloned().collect();

    let mut unvisited: PriorityQueue<(usize, usize), Reverse<usize>> = PriorityQueue::new();
    let mut visited: Vec<((usize, usize), usize)> = Vec::new();
    for x in 0..=max_dim {
        for y in 0..=max_dim {
            let pos = (x, y);
            if !positions.contains(&pos) {
                unvisited.push(pos, Reverse(usize::MAX));
            }
        }
    }

    // Ensure start node is popped first
    unvisited.change_priority(&(0, 0), Reverse(0));

    // Dijkstra
    while let Some((current, Reverse(cost))) = unvisited.pop() {
        // Break if only unreachable nodes are rerun_outer() -> Stringing
        if cost == usize::MAX {
            break;
        }

        for neighbor in neighbor::NeighborIter::new(current, max_dim) {
            let new_cost = cost + 1;
            if let Some(Reverse(existing_cost)) = unvisited.get_priority(&neighbor) {
                if new_cost < *existing_cost {
                    unvisited.change_priority(&neighbor, Reverse(new_cost));
                }
            }
        }

        visited.push((current, cost));
    }

    // Find end node in visited Vec, and check cost
    for (pos, cost) in visited.iter() {
        if *pos == (max_dim, max_dim) {
            return Some(*cost as u64);
        }
    }

    None
}

fn run_inner(input: &str, max_dim: usize, sim_limit: usize) -> (u64, String) {
    let all_positions: Vec<(usize, usize)> = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let pt1 = shortest_path(&all_positions, max_dim, sim_limit).unwrap();

    // For part 2, binary search the input to find the point at which the end becomes unreachable
    let mut low = 0;
    let mut high = all_positions.len();

    while low < high {
        let mid = (low + high) / 2;

        if let Some(_) = shortest_path(&all_positions, max_dim, mid) {
            // Possible to reach end, look again in high side
            low = mid + 1;
        } else {
            // Not possible to reach end, look again in low side
            high = mid;
        }
    }

    // Double check that we have the right value...
    assert!(shortest_path(&all_positions, max_dim, low).is_none());
    assert!(shortest_path(&all_positions, max_dim, low - 1).is_some());

    // We found the length of input at which the end becomes unreachable. The index at which the value
    // causing the end to be unreachable lies is 1 less than this.
    let pt2 = all_positions[low - 1];
    let pt2_fmt = format!("{},{}", pt2.0, pt2.1);

    (pt1, pt2_fmt)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/18.ex");
        let (pt1, pt2) = run_inner(&input, 6, 12);
        assert_eq!(pt1, 22);
        assert_eq!(pt2, "6,1");
    }
}
