use std::{collections::HashSet, time::Instant};

fn main() {
    let input = include_str!("../../inputs/14.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Clone)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let (p, v) = s.split_once(' ').unwrap();

        let (_, p) = p.split_at(2);
        let (_, v) = v.split_at(2);

        let (px, py) = p.split_once(',').unwrap();
        let (vx, vy) = v.split_once(',').unwrap();

        Robot {
            position: (px.parse().unwrap(), py.parse().unwrap()),
            velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}

impl Robot {
    fn step_by(&self, n: isize, bounds: (isize, isize)) -> (isize, isize) {
        let x = self.position.0 + n * self.velocity.0;
        let y = self.position.1 + n * self.velocity.1;

        (x.rem_euclid(bounds.0), y.rem_euclid(bounds.1))
    }
}

fn count_positions_in_range(
    positions: &Vec<(isize, isize)>,
    min_bound: (isize, isize),
    max_bound: (isize, isize),
) -> u64 {
    positions
        .iter()
        .map(|pos| {
            if pos.0 >= min_bound.0
                && pos.1 >= min_bound.1
                && pos.0 < max_bound.0
                && pos.1 < max_bound.1
            {
                1
            } else {
                0
            }
        })
        .sum()
}

fn display_robots(positions: &Vec<(isize, isize)>) {
    let mut minx = isize::MAX;
    let mut miny = isize::MAX;
    let mut maxx = 0;
    let mut maxy = 0;

    for pos in positions.iter() {
        if pos.0 < minx {
            minx = pos.0;
        }
        if pos.0 > maxx {
            maxx = pos.0;
        }
        if pos.1 < miny {
            miny = pos.1;
        }
        if pos.1 > maxy {
            maxy = pos.1;
        }
    }

    for y in miny..maxy + 1 {
        for x in minx..maxx + 1 {
            if positions.iter().any(|pos| pos.0 == x && pos.1 == y) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn run_inner(input: &str, steps: isize, bounds: (isize, isize)) -> (u64, u64) {
    let robots: Vec<Robot> = input.trim().lines().map(|line| Robot::from(line)).collect();

    let end_positions: Vec<(isize, isize)> =
        robots.iter().map(|r| r.step_by(steps, bounds)).collect();

    let midpoint_x = bounds.0 / 2;
    let midpoint_y = bounds.1 / 2;

    let quadrant_counts = [
        count_positions_in_range(&end_positions, (0, 0), (midpoint_x, midpoint_y)),
        count_positions_in_range(&end_positions, (midpoint_x + 1, 0), (bounds.0, midpoint_y)),
        count_positions_in_range(&end_positions, (0, midpoint_y + 1), (midpoint_x, bounds.1)),
        count_positions_in_range(
            &end_positions,
            (midpoint_x + 1, midpoint_y + 1),
            (bounds.0, bounds.1),
        ),
    ];

    let pt1: u64 = quadrant_counts.iter().product();

    // Part 2 is a weird one. We don't actually know the image that we are looking for. There's
    // a variety of heuristics that we could use, but ultimately we have to print out the layout
    // and confirm if we were right or not. It's probably fair to assume that the image won't have
    // overlapping robots since these would not add to the image. So try that first.
    let mut pt2 = 0u64;
    for i in 1.. {
        let end_positions: Vec<(isize, isize)> =
            robots.iter().map(|r| r.step_by(i, bounds)).collect();

        if end_positions.len() == HashSet::<&(isize, isize)>::from_iter(end_positions.iter()).len()
        {
            display_robots(&end_positions);
            pt2 = i as u64;
            break;
        }
    }

    (pt1, pt2)
}

fn run(input: &str) -> (u64, u64) {
    run_inner(input, 100, (101, 103))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/14.ex");
        let (pt1, pt2) = run_inner(&input, 100, (11, 7));
        assert_eq!(pt1, 12);
        assert_eq!(pt2, 0);
    }

    #[test]
    fn test_step_by() {
        let robot = Robot {
            position: (2, 4),
            velocity: (2, -3),
        };
        let end_pos = robot.step_by(5, (11, 7));
        assert_eq!(end_pos, (1, 3));
    }
}
