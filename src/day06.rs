use rayon::prelude::*;
use std::{collections::HashSet, fmt::Display, time::Instant};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/06.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed)
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_numerical(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GuardState {
    Patrolling,
    LeftArea,
    InfiniteLoop,
}

#[derive(Debug, Clone)]
struct GridState {
    /// Dimensions of the whole grid: (x, y)
    dimensions: (usize, usize),

    /// Current position of the guard in the grid: (x, y)
    guard_position: (usize, usize),

    /// Current direction the guard is facing
    guard_direction: Direction,

    /// Obstacles in the grid
    obstacles: HashSet<(usize, usize)>,

    /// Locations visited
    visited: HashSet<(usize, usize, Direction)>,
}

impl GridState {
    fn parse(input: &str) -> GridState {
        let mut dimensions = (0, 0);
        let mut guard_position: Option<(usize, usize)> = None;
        let mut guard_direction: Option<Direction> = None;
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();

        for (y, line) in input.trim().lines().enumerate() {
            // Check if the guard is in this line
            if guard_position.is_none() {
                use Direction::*;

                for dir in [Up, Down, Left, Right] {
                    let c: char = dir.into();
                    if let Some(x) = line.find(c) {
                        guard_position = Some((x, y));
                        guard_direction = Some(dir);
                    }
                }
            }

            // Find all obstacles in this line
            for (x, _) in line.match_indices('#') {
                obstacles.insert((x, y));
            }

            // Update dimensions
            dimensions.0 = line.len();
            dimensions.1 = y + 1;
        }

        let guard_position = guard_position.unwrap();
        let guard_direction = guard_direction.unwrap();

        let mut visited = HashSet::new();
        visited.insert((guard_position.0, guard_position.1, guard_direction));

        GridState {
            dimensions,
            guard_position,
            guard_direction,
            obstacles,
            visited,
        }
    }

    fn would_exceed_bounds(&self, movement: (isize, isize)) -> bool {
        let maxx = self.dimensions.0 - 1;
        let maxy = self.dimensions.1 - 1;

        match (self.guard_position, movement) {
            ((0, _), (-1, _)) => true,
            ((_, 0), (_, -1)) => true,
            ((x, _), (1, _)) if x == maxx => true,
            ((_, y), (_, 1)) if y == maxy => true,
            _ => false,
        }
    }

    fn step(&mut self) -> GuardState {
        let (x, y) = self.guard_position;
        let (dx, dy) = self.guard_direction.to_numerical();

        if self.would_exceed_bounds((dx, dy)) {
            // We're done, guard goes out of bounds
            return GuardState::LeftArea;
        }

        // We have already bounds checked this
        let newx = (x as isize + dx) as usize;
        let newy = (y as isize + dy) as usize;

        if self.obstacles.contains(&(newx, newy)) {
            self.guard_direction = self.guard_direction.next();
        } else {
            self.guard_position = (newx, newy);
        }

        if !self.visited.insert((
            self.guard_position.0,
            self.guard_position.1,
            self.guard_direction,
        )) {
            // Position and direction was already visited, so we have an infinite loop
            return GuardState::InfiniteLoop;
        }

        GuardState::Patrolling
    }

    fn step_until_end(&mut self) -> GuardState {
        loop {
            let guard_state = self.step();
            if guard_state != GuardState::Patrolling {
                return guard_state;
            }
        }
    }

    fn unique_positions(&self) -> HashSet<(usize, usize)> {
        self.visited.iter().map(|(x, y, _)| (*x, *y)).collect()
    }

    fn count_visited(&self) -> usize {
        self.unique_positions().len()
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unique_positions = self.unique_positions();

        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                if self.obstacles.contains(&(x, y)) {
                    write!(f, "#")?;
                } else if (x, y) == self.guard_position {
                    match self.guard_direction {
                        Direction::Up => write!(f, "^")?,
                        Direction::Right => write!(f, ">")?,
                        Direction::Down => write!(f, "v")?,
                        Direction::Left => write!(f, "<")?,
                    };
                } else if unique_positions.contains(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn run(input: &str) -> (u64, u64) {
    let initial_grid_state = GridState::parse(input);

    let mut grid_state = initial_grid_state.clone();
    let end_state = grid_state.step_until_end();
    assert!(end_state == GuardState::LeftArea);

    let pt1 = grid_state.count_visited() as u64;

    // For part 2, we only need to try adding obstacles at positions that were visited in part 1.
    // Otherwise the guard will never hit the obstacle. We also need to remove the guard's initial
    // position from the list of places to try putting obstacles.
    let mut possible_obstacle_locations = grid_state.unique_positions();
    possible_obstacle_locations.remove(&initial_grid_state.guard_position);
    let possible_obstacle_locations = possible_obstacle_locations; // no longer needs to be mutable

    // To parallelise calculations, we need a vec rather than a hashset
    let possible_locations_vec: Vec<(usize, usize)> = possible_obstacle_locations
        .iter()
        .map(|item| *item)
        .collect();

    let pt2 = possible_locations_vec
        .into_par_iter()
        .filter(|(x, y)| {
            // Create a new grid and insert the obstacle
            let mut gs = initial_grid_state.clone();
            gs.obstacles.insert((*x, *y));

            let end_state = gs.step_until_end();

            end_state == GuardState::InfiniteLoop
        })
        .count() as u64;

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/06.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 41);
        assert_eq!(pt2, 6);
    }
}
