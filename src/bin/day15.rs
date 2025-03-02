use std::{fmt::Display, time::Instant};

fn main() {
    let input = include_str!("../../inputs/15.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Copy, Clone)]
struct MyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = MyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Move::Up),
            '>' => Ok(Move::Right),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            _ => Err(MyError),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GridElement {
    Wall,
    Free,
    Box,
    Robot,
}

impl TryFrom<char> for GridElement {
    type Error = MyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(GridElement::Wall),
            '.' => Ok(GridElement::Free),
            'O' => Ok(GridElement::Box),
            '@' => Ok(GridElement::Robot),
            _ => Err(MyError),
        }
    }
}

impl Into<char> for GridElement {
    fn into(self) -> char {
        match self {
            GridElement::Wall => '#',
            GridElement::Free => '.',
            GridElement::Robot => '@',
            GridElement::Box => 'O',
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    elements: Vec<Vec<GridElement>>,
    robot_loc: (usize, usize),
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let mut robot_loc = (0usize, 0usize);

        let elements: Vec<Vec<GridElement>> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        let ge = GridElement::try_from(ch).unwrap();
                        if ge == GridElement::Robot {
                            robot_loc = (x, y);
                        }
                        ge
                    })
                    .collect()
            })
            .collect();

        Grid {
            elements,
            robot_loc,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.elements.iter() {
            for ge in line.iter() {
                write!(f, "{}", Into::<char>::into(*ge))?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl Grid {
    fn apply_move(&mut self, mv: Move) {
        let (next_x, next_y) = match mv {
            Move::Up => (self.robot_loc.0, self.robot_loc.1 - 1),
            Move::Down => (self.robot_loc.0, self.robot_loc.1 + 1),
            Move::Right => (self.robot_loc.0 + 1, self.robot_loc.1),
            Move::Left => (self.robot_loc.0 - 1, self.robot_loc.1),
        };

        let next_element = self.elements[next_y][next_x];

        match next_element {
            GridElement::Free => {
                self.elements[next_y][next_x] = GridElement::Robot;
                self.elements[self.robot_loc.1][self.robot_loc.0] = GridElement::Free;
                self.robot_loc = (next_x, next_y);
            }
            GridElement::Wall => {
                // Can't move
            }
            GridElement::Box => {
                // Now we need to see if the box can be pushed forward...
                // Check grid squares in the relevant direction until we find:
                // - a free square: in which case put a box there an move the robot forward
                // - a wall: in which case we cannot move and should stop checking
                let (mut x, mut y) = (next_x, next_y);
                match mv {
                    Move::Up => {
                        while let GridElement::Box = self.elements[y][x] {
                            y -= 1;
                        }
                    }
                    Move::Down => {
                        while let GridElement::Box = self.elements[y][x] {
                            y += 1;
                        }
                    }
                    Move::Right => {
                        while let GridElement::Box = self.elements[y][x] {
                            x += 1;
                        }
                    }
                    Move::Left => {
                        while let GridElement::Box = self.elements[y][x] {
                            x -= 1;
                        }
                    }
                }

                match self.elements[y][x] {
                    GridElement::Wall => {
                        // Can't move, do nothing
                    }
                    GridElement::Free => {
                        // Place a box here and move the robot forward
                        self.elements[y][x] = GridElement::Box;
                        self.elements[next_y][next_x] = GridElement::Robot;
                        self.elements[self.robot_loc.1][self.robot_loc.0] = GridElement::Free;
                        self.robot_loc = (next_x, next_y);
                    }
                    _ => panic!("There shouldn't be a box or a robot here..."),
                }
            }
            GridElement::Robot => {
                panic!("Can't be more than one robot...");
            }
        };
    }

    fn gps_score(&self) -> u64 {
        self.elements
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, ge)| {
                    if let GridElement::Box = ge {
                        Some((100 * y + x) as u64)
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .sum()
    }
}

fn run(input: &str) -> (u64, u64) {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let grid = Grid::from(grid);
    let moves: Vec<Move> = moves
        .chars()
        .filter_map(|ch| Move::try_from(ch).ok())
        .collect();

    let mut g = grid.clone();
    for m in moves.iter() {
        g.apply_move(*m);
    }

    let pt1 = g.gps_score();

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/15.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 10092);
        assert_eq!(pt2, 0);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../../inputs/15_2.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 2028);
        assert_eq!(pt2, 0);
    }
}
