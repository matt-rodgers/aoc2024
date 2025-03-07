use std::{cmp::Reverse, collections::HashSet, time::Instant};

use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../../inputs/18.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, String) {
    run_inner(input, 70, 1024)
}

struct NeighborIter {
    idx: usize,
    vals: [Option<(usize, usize)>; 4],
}

impl NeighborIter {
    fn new(pos: (usize, usize), max_dim: usize) -> Self {
        let mut vals = [None; 4];

        if pos.0 > 0 {
            vals[0] = Some((pos.0 - 1, pos.1));
        }

        if pos.0 < max_dim {
            vals[1] = Some((pos.0 + 1, pos.1));
        }

        if pos.1 > 0 {
            vals[2] = Some((pos.0, pos.1 - 1));
        }

        if pos.1 < max_dim {
            vals[3] = Some((pos.0, pos.1 + 1));
        }

        Self { idx: 0, vals }
    }
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.vals.len() {
            let val = self.vals[self.idx];
            self.idx += 1;

            if val.is_some() {
                return val;
            }
        }

        None
    }
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
        // Break if only unreachable nodes are remaining
        if cost == usize::MAX {
            break;
        }

        for neighbor in NeighborIter::new(current, max_dim) {
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
        println!("low: {}, mid: {}, high: {}", low, mid, high);

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
    use std::fmt::Debug;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/18.ex");
        let (pt1, pt2) = run_inner(&input, 6, 12);
        assert_eq!(pt1, 22);
        assert_eq!(pt2, "6,1");
    }

    fn assert_equal_unordered<T>(mut a: Vec<T>, mut b: Vec<T>)
    where
        T: Ord + Debug,
    {
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn test_neighbor_iter() {
        let ni = NeighborIter::new((1, 1), 2);
        let vals: Vec<(usize, usize)> = ni.collect();
        assert_equal_unordered(vals, vec![(0, 1), (1, 0), (2, 1), (1, 2)]);

        let ni = NeighborIter::new((0, 0), 2);
        let vals: Vec<(usize, usize)> = ni.collect();
        assert_equal_unordered(vals, vec![(0, 1), (1, 0)]);
    }
}
