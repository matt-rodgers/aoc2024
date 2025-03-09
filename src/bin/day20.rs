use neighbor::NeighborIter;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap, time::Instant};

mod neighbor;

fn main() {
    let input = include_str!("../../inputs/20.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    run_inner(input, 100)
}

fn run_inner(input: &str, threshold: u32) -> (u64, u64) {
    let mut end = (0, 0);
    let mut unvisited: PriorityQueue<(usize, usize), Reverse<u32>> = PriorityQueue::new();
    let mut wall_tiles: Vec<(usize, usize)> = Vec::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    unvisited.push((x, y), Reverse(u32::MAX));
                }
                'S' => {
                    unvisited.push((x, y), Reverse(0));
                }
                'E' => {
                    unvisited.push((x, y), Reverse(u32::MAX));
                    end = (x, y);
                }
                '#' => {
                    wall_tiles.push((x, y));
                }
                _ => {}
            };
        }
    }

    // Find the path through the course without any cheats, recording the time taken to get to
    // each position.
    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();

    while let Some((current, Reverse(cost))) = unvisited.pop() {
        // Break if all remaining nodes are unreachable
        if cost == u32::MAX {
            break;
        }

        for neighbor in NeighborIter::new(current, usize::MAX) {
            let new_cost = cost + 1;

            // Check if the neighbor is still unvisited
            if let Some(Reverse(existing_cost)) = unvisited.get_priority(&neighbor) {
                // Update cost if we have found a better path to this node
                if new_cost < *existing_cost {
                    unvisited.change_priority(&neighbor, Reverse(new_cost));
                }
            }
        }

        visited.insert(current, cost);
    }

    // Verify that the end node has the maximum cost. If not, then we need to think again about how
    // to check the time saved by a shortcut...
    let mut max_cost = 0;
    let mut max_cost_pos = (0, 0);
    for (k, v) in visited.iter() {
        if *v > max_cost {
            max_cost = *v;
            max_cost_pos = *k;
        }
    }
    assert_eq!(max_cost_pos, end);

    // Then for each two distinct positions on the course separated by a single wall tile, check
    // how much time would be saved by going through the wall.
    let mut pt1 = 0;
    for wall_tile in wall_tiles.iter() {
        // Check if the wall tile separates two positions on the course, vertically or horizontally
        if wall_tile.0 > 0 {
            if let Some(time_saved) = time_saved_by_shortcut(
                &(wall_tile.0 - 1, wall_tile.1),
                &(wall_tile.0 + 1, wall_tile.1),
                &visited,
            ) {
                if time_saved >= threshold {
                    pt1 += 1;
                }
            }
        }

        if wall_tile.1 > 0 {
            if let Some(time_saved) = time_saved_by_shortcut(
                &(wall_tile.0, wall_tile.1 - 1),
                &(wall_tile.0, wall_tile.1 + 1),
                &visited,
            ) {
                if time_saved >= threshold {
                    pt1 += 1;
                }
            }
        }
    }

    (pt1, 0)
}

fn time_saved_by_shortcut(
    a: &(usize, usize),
    b: &(usize, usize),
    visited: &HashMap<(usize, usize), u32>,
) -> Option<u32> {
    if let Some(ta) = visited.get(a) {
        if let Some(tb) = visited.get(b) {
            let diff = ta.abs_diff(*tb);
            let time_saved = diff.checked_sub(2).unwrap(); // diff should be > 2
            return Some(time_saved);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/20.ex");
        let (pt1, pt2) = run_inner(&input, 10);
        assert_eq!(pt1, 10);
        assert_eq!(pt2, 0);
    }
}
