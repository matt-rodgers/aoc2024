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
    let pt1 = run_inner(input, 2, 100);
    let pt2 = run_inner(input, 20, 100);
    (pt1, pt2)
}

fn run_inner(input: &str, cheat_time: usize, threshold: u32) -> u64 {
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

    // For each position that has been visited, check which of the other visited positions can be
    // reached within the cheat time (by moving freely anywhere on the grid, not just along points
    // part of the path). For each of these reachable positions, check the time saving (if any).
    let mut res = 0;
    for (pos, cost) in visited.iter() {
        for other in points_within_distance(*pos, cheat_time) {
            // Check if the other point is reachable from course
            if let Some(other_cost) = visited.get(&other) {
                let d = distance(&pos, &other);

                // We can only save time if the other point has a lower cost than this point AND
                // this difference in cost is greater than the distance travelled via the shortcut
                let new_other_cost = other_cost + d as u32;
                if new_other_cost < *cost {
                    let time_saved = cost - new_other_cost;
                    if time_saved >= threshold {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn points_within_distance(
    (x, y): (usize, usize),
    distance: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let miny = y.saturating_sub(distance);
    let maxy = y.saturating_add(distance);

    (miny..=maxy).flat_map(move |ny| {
        let rem = distance - y.abs_diff(ny);
        let minx = x.saturating_sub(rem);
        let maxx = x.saturating_add(rem);
        (minx..=maxx).map(move |nx| (nx, ny))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/20.ex");
        let pt1 = run_inner(&input, 2, 10);
        assert_eq!(pt1, 10);

        let pt2 = run_inner(&input, 20, 50);
        assert_eq!(pt2, 285);
    }

    #[test]
    fn test_points_within_distance() {
        let mut actual: Vec<(usize, usize)> = points_within_distance((2, 2), 1).collect();
        actual.sort();
        let mut expected = vec![(1, 2), (2, 2), (3, 2), (2, 1), (2, 3)];
        expected.sort();
        assert_eq!(actual, expected);

        let mut actual: Vec<(usize, usize)> = points_within_distance((2, 2), 2).collect();
        actual.sort();

        // Starting point is '.', reachable is '#', not reachable is 'x'
        //   x x # x x
        //   x # # # x
        //   # # . # #
        //   x # # # x
        //   x x # x x
        let mut expected = vec![
            (2, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (2, 4),
        ];
        expected.sort();
        assert_eq!(actual, expected);
    }
}
