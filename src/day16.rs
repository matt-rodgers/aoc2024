use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    time::Instant,
};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/16.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!(
        "pt1: {} , pt2: {} , elapsed time {:?} us",
        pt1,
        pt2,
        elapsed.as_micros()
    )
}

const MOVE_COST: usize = 1;
const TURN_COST: usize = 1000;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum Dir {
    #[default]
    North,
    East,
    South,
    West,
}

impl Dir {
    fn turn_cw(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

const ALL_DIRECTIONS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Node {
    dir: Dir,
    x: isize,
    y: isize,
}

struct AdjacentNodeIter {
    start: Node,
    count: usize,
}

impl Iterator for AdjacentNodeIter {
    type Item = (Node, usize); // Node, cost

    fn next(&mut self) -> Option<Self::Item> {
        let (item, cost) = match self.count {
            0 => match self.start.dir {
                Dir::North => (
                    Node {
                        y: self.start.y - 1,
                        ..self.start
                    },
                    MOVE_COST,
                ),
                Dir::East => (
                    Node {
                        x: self.start.x + 1,
                        ..self.start
                    },
                    MOVE_COST,
                ),
                Dir::South => (
                    Node {
                        y: self.start.y + 1,
                        ..self.start
                    },
                    MOVE_COST,
                ),
                Dir::West => (
                    Node {
                        x: self.start.x - 1,
                        ..self.start
                    },
                    MOVE_COST,
                ),
            },
            1 => (
                Node {
                    dir: self.start.dir.turn_cw(),
                    ..self.start
                },
                TURN_COST,
            ),
            2 => (
                Node {
                    dir: self.start.dir.turn_cw().turn_cw(),
                    ..self.start
                },
                2 * TURN_COST,
            ),
            3 => (
                Node {
                    dir: self.start.dir.turn_cw().turn_cw().turn_cw(),
                    ..self.start
                },
                TURN_COST,
            ),
            _ => return None,
        };

        self.count += 1;
        Some((item, cost))
    }
}

impl Node {
    fn adjacent(&self) -> AdjacentNodeIter {
        AdjacentNodeIter {
            start: self.clone(),
            count: 0,
        }
    }
}

fn run(input: &str) -> (u64, u64) {
    let mut start_node = Node::default();
    let mut end = (0, 0);
    let mut nodes: Vec<Node> = Vec::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let should_insert = match ch {
                '.' => true,
                'S' => {
                    start_node = Node {
                        dir: Dir::East,
                        x: x as isize,
                        y: y as isize,
                    };
                    true
                }
                'E' => {
                    end = (x as isize, y as isize);
                    true
                }
                _ => false,
            };

            if should_insert {
                for dir in ALL_DIRECTIONS.iter() {
                    nodes.push(Node {
                        dir: *dir,
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }
    }

    let mut unvisited: PriorityQueue<Node, Reverse<usize>> = nodes
        .iter()
        .map(|n| {
            if *n == start_node {
                (*n, Reverse(0))
            } else {
                (*n, Reverse(usize::MAX))
            }
        })
        .collect();
    let mut visited: Vec<(Node, usize)> = Vec::new();
    let mut prev: HashMap<Node, Vec<Node>> = HashMap::new();

    while let Some((current, Reverse(cost))) = unvisited.pop() {
        // End condition of loop: we might not be able to visit all nodes, and need to break if the
        // node we pop has an infinite cost
        if cost == usize::MAX {
            break;
        }

        for (neighbor, additional_cost) in current.adjacent() {
            let new_cost = cost + additional_cost;
            if let Some(Reverse(existing_cost)) = unvisited.get_priority(&neighbor) {
                if new_cost < *existing_cost {
                    // Update the cost of this node
                    unvisited.change_priority(&neighbor, Reverse(new_cost));

                    // Replace the existing previous node(s) of the unvisited node with the
                    // current node
                    prev.insert(neighbor, vec![current]);
                } else if new_cost == *existing_cost {
                    // If the cost is equal, append to the array of previous nodes (since there
                    // is now more than one lowest-cost previous node)
                    let p = prev
                        .get_mut(&neighbor)
                        .expect("Neighbor should already have an entry in prev");
                    p.push(current);
                }
            }
        }

        visited.push((current, cost));
    }

    // Find the end node in the visited array, and check the min cost. Note that we will actually
    // have four end nodes (one for each direction we can face in), any of these is an acceptable
    // end point so check all of them for the min cost.
    let mut pt1 = usize::MAX;
    let mut end_node: Option<Node> = None;
    for (node, cost) in visited.iter() {
        if (node.x, node.y) == end {
            if *cost < pt1 {
                end_node = Some(*node);
                pt1 = *cost;
            }
        }
    }

    // Traverse the map of previous nodes, starting from the end node until we get to the start
    // node. We need to count up the nodes with unique locations (the same location but different
    // direction does not count as an additional node on the best path)
    let end_node = end_node.unwrap();
    let mut pending_nodes: Vec<Node> = vec![end_node];
    let mut best_path: HashSet<(isize, isize)> = HashSet::new();
    best_path.insert((end_node.x, end_node.y));

    while let Some(next) = pending_nodes.pop() {
        let new_nodes = prev
            .get(&next)
            .expect("Any node in the pending list should have a previous node");
        for node in new_nodes {
            best_path.insert((node.x, node.y));
            if *node != start_node {
                pending_nodes.push(*node);
            }
        }
    }

    let pt2 = best_path.len() as u64;

    (pt1 as u64, pt2)
}

#[cfg(test)]
mod test {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/16.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 7036);
        assert_eq!(pt2, 45);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../inputs/16_2.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 11048);
        assert_eq!(pt2, 64);
    }

    #[test]
    fn test_adjacent_iter() {
        let node = Node {
            dir: Dir::North,
            x: 0,
            y: 0,
        };
        let mut expected = vec![
            (
                Node {
                    dir: Dir::North,
                    x: 0,
                    y: -1,
                },
                1,
            ),
            (
                Node {
                    dir: Dir::East,
                    x: 0,
                    y: 0,
                },
                1000,
            ),
            (
                Node {
                    dir: Dir::South,
                    x: 0,
                    y: 0,
                },
                2000,
            ),
            (
                Node {
                    dir: Dir::West,
                    x: 0,
                    y: 0,
                },
                1000,
            ),
        ];

        let mut actual: Vec<(Node, usize)> = node.adjacent().collect();

        expected.sort();
        actual.sort();

        assert_equal(actual, expected);
    }
}
