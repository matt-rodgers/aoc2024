use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let input = include_str!("../../inputs/10.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const ALL_DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_one_step(&self, dir: Dir) -> Point {
        match dir {
            Dir::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn find_reachable_summits(
    pos: &Point,
    height: u32,
    grid: &HashMap<Point, u32>,
    summits: &mut HashSet<Point>,
) -> u64 {
    let mut trail_count = 0;

    for d in ALL_DIRECTIONS.iter() {
        let next_pos = pos.move_one_step(*d);

        // Check if next position exists in grid
        if let Some(next_height) = grid.get(&next_pos) {
            // Check if it's possible to move to this position
            if *next_height == height + 1 {
                // Check end condition of recursion
                if *next_height == 9 {
                    summits.insert(next_pos);
                    trail_count += 1;
                } else {
                    // Continue search from next position
                    trail_count += find_reachable_summits(&next_pos, *next_height, grid, summits);
                }
            }
        }
    }

    trail_count
}

fn run(input: &str) -> (u64, u64) {
    let mut grid = HashMap::<Point, u32>::new();
    let mut trailheads = HashSet::<Point>::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = Point {
                x: x as isize,
                y: y as isize,
            };
            let n = ch.to_digit(10).unwrap();

            grid.insert(pos, n);

            if n == 0 {
                trailheads.insert(pos);
            }
        }
    }

    let mut pt1: u64 = 0;
    let mut pt2: u64 = 0;

    for tr in trailheads.iter() {
        let mut reachable_summits = HashSet::new();
        let trail_count = find_reachable_summits(tr, 0, &grid, &mut reachable_summits);
        pt1 += reachable_summits.len() as u64;
        pt2 += trail_count;
    }

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/10.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 36);
        assert_eq!(pt2, 81);
    }
}
