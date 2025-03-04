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
    BoxL,
    BoxR,
}

impl TryFrom<char> for GridElement {
    type Error = MyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(GridElement::Wall),
            '.' => Ok(GridElement::Free),
            'O' => Ok(GridElement::Box),
            '@' => Ok(GridElement::Robot),
            '[' => Ok(GridElement::BoxL),
            ']' => Ok(GridElement::BoxR),
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
            GridElement::BoxL => '[',
            GridElement::BoxR => ']',
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
    fn apply_move_inner(&mut self, (x, y): (usize, usize), mv: Move) -> Result<(), MyError> {
        let (nx, ny) = match mv {
            Move::Up => (x, y - 1),
            Move::Down => (x, y + 1),
            Move::Right => (x + 1, y),
            Move::Left => (x - 1, y),
        };

        let current_element = self.elements[y][x];
        let next_element = self.elements[ny][nx];

        let res = match (next_element, mv) {
            (GridElement::Wall, _) => return Err(MyError),
            (GridElement::Free, _) => Ok(()),
            (GridElement::Robot, _) => panic!("Multiple robots in grid!"),
            (GridElement::BoxL | GridElement::BoxR, Move::Up | Move::Down) => {
                let offset: isize = if next_element == GridElement::BoxL {
                    1
                } else {
                    -1
                };

                let r1 = self.apply_move_inner((nx, ny), mv);
                let r2 = self.apply_move_inner(((nx as isize + offset) as usize, ny), mv);

                match (r1, r2) {
                    (Ok(_), Ok(_)) => Ok(()),
                    _ => Err(MyError),
                }
            }
            (_, _) => self.apply_move_inner((nx, ny), mv),
        };

        // Element at current location might not actually end up free. But if this is the case it
        // will be overwritten with the correct value after this function returns.
        self.elements[y][x] = GridElement::Free;
        self.elements[ny][nx] = current_element;

        if current_element == GridElement::Robot {
            self.robot_loc = (nx, ny);
        }

        res
    }

    fn apply_move(&mut self, mv: Move) {
        let mut new_self = self.clone();

        if new_self.apply_move_inner(self.robot_loc, mv).is_ok() {
            self.elements = new_self.elements;
            self.robot_loc = new_self.robot_loc;
        }
    }

    fn gps_score(&self) -> u64 {
        self.elements
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, ge)| {
                    if let GridElement::Box | GridElement::BoxL = ge {
                        Some((100 * y + x) as u64)
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .sum()
    }

    fn expanded(&self) -> Self {
        let new_elements: Vec<Vec<GridElement>> = self
            .elements
            .iter()
            .map(|line| {
                line.iter()
                    .flat_map(|ge| match ge {
                        GridElement::Wall => vec![GridElement::Wall, GridElement::Wall],
                        GridElement::Free => vec![GridElement::Free, GridElement::Free],
                        GridElement::Robot => vec![GridElement::Robot, GridElement::Free],
                        GridElement::Box => vec![GridElement::BoxL, GridElement::BoxR],
                        _ => panic!("Cannot expand an already expanded grid"),
                    })
                    .collect()
            })
            .collect();

        Self {
            elements: new_elements,
            robot_loc: (self.robot_loc.0 * 2, self.robot_loc.1),
        }
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

    let mut expanded_grid = grid.expanded();
    for m in moves.iter() {
        expanded_grid.apply_move(*m);
    }

    let pt2 = expanded_grid.gps_score();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/15.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 10092);
        assert_eq!(pt2, 9021);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../../inputs/15_2.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 2028);
    }
}
