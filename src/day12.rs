use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
    time::Instant,
};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/12.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<(isize, isize)> for Point {
    type Output = Point;

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

fn run(input: &str) -> (u64, u64) {
    let points = parse_input(input);
    let mut unvisited = points.clone();
    let mut pt1 = 0usize;
    let mut pt2 = 0usize;

    while !unvisited.is_empty() {
        let (start, variant) = unvisited.iter().next().unwrap();
        let start = start.clone();
        let variant = variant.clone();
        let mut inside = HashSet::new();
        let mut boundary = Vec::new();

        flood_fill(
            &start,
            variant,
            &points,
            &mut unvisited,
            &mut inside,
            &mut boundary,
        );

        pt1 += inside.len() * boundary.len();

        // The number of vertices is equal to the number of sides
        let nvertices = count_vertices(&inside);
        pt2 += inside.len() * nvertices;
    }

    (pt1 as u64, pt2 as u64)
}

fn parse_input(input: &str) -> HashMap<Point, char> {
    let mut points = HashMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            points.insert(
                Point {
                    x: x as isize,
                    y: y as isize,
                },
                ch,
            );
        }
    }

    points
}

fn flood_fill(
    node: &Point,
    variant: char,
    points: &HashMap<Point, char>,
    unvisited: &mut HashMap<Point, char>,
    inside: &mut HashSet<Point>,
    boundary: &mut Vec<Point>, // Boundary points can be counted more than once
) {
    if let Some(ch) = points.get(node) {
        if *ch != variant {
            boundary.push(*node);
            return;
        }

        inside.insert(*node);
        unvisited.remove(node);

        for new_node in [
            Point {
                x: node.x + 1,
                y: node.y,
            },
            Point {
                x: node.x - 1,
                y: node.y,
            },
            Point {
                x: node.x,
                y: node.y + 1,
            },
            Point {
                x: node.x,
                y: node.y - 1,
            },
        ]
        .iter()
        {
            if !inside.contains(&new_node) {
                flood_fill(new_node, variant, points, unvisited, inside, boundary);
            }
        }
    } else {
        // Add to boundary even if outside of the grid
        boundary.push(*node);
    }
}

const DIAGONALS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn count_vertices(inside: &HashSet<Point>) -> usize {
    inside
        .iter()
        .map(|pt| {
            let mut nvertices = 0;

            for diag in DIAGONALS.iter() {
                // Get the points adjacent to the corner.
                // If the current point is P, then a, b, and c are shown below:
                //
                //     a b    or    P a    etc...
                //     P c          c b
                //
                let (a, b, c) = (*pt + (diag.0, 0), *pt + *diag, *pt + (0, diag.1));

                let a_inside = inside.contains(&a);
                let c_inside = inside.contains(&c);

                // If the two "non-diagonal" points are both not contained in the inside, then we have
                // an "outer" vertex, like this (I=inside, O=outside)
                //
                //    I O   or    I O (for an unusual shape which touches itself again)
                //    O O         O I
                //
                if !a_inside && !c_inside {
                    nvertices += 1;
                }

                // If the two "non-diagonal" points are both contained in the inside AND the diagonal
                // point is not inside, then we have an "inner" vertex, like this (I=inside, O=outside)
                //
                //   I I
                //   I O
                //
                if a_inside && c_inside && !inside.contains(&b) {
                    nvertices += 1;
                }
            }

            nvertices
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flood_one_area() {
        let input = include_str!("../inputs/12.ex");
        let points = parse_input(&input);
        let mut unvisited = points.clone();

        let start = Point { x: 0, y: 0 };
        let mut inside = HashSet::new();
        let mut boundary = Vec::new();
        let variant = points.get(&start).unwrap();

        flood_fill(
            &start,
            *variant,
            &points,
            &mut unvisited,
            &mut inside,
            &mut boundary,
        );

        assert_eq!(inside.len(), 4);
        assert_eq!(boundary.len(), 10);
        assert_eq!(unvisited.len(), points.len() - inside.len());
    }

    #[test]
    fn test_example_1() {
        let input = include_str!("../inputs/12.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 140);
        assert_eq!(pt2, 80);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../inputs/12_2.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 772);
        assert_eq!(pt2, 436);
    }

    #[test]
    fn test_example_3() {
        let input = include_str!("../inputs/12_3.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 1930);
        assert_eq!(pt2, 1206);
    }

    #[test]
    fn test_example_4() {
        let input = include_str!("../inputs/12_4.ex");
        let (_pt1, pt2) = run(&input);
        assert_eq!(pt2, 236);
    }

    #[test]
    fn test_example_5() {
        let input = include_str!("../inputs/12_5.ex");
        let (_pt1, pt2) = run(&input);
        assert_eq!(pt2, 368);
    }

    #[test]
    fn test_count_vertices() {
        // A simple square of area 1
        let mut inside = HashSet::new();
        inside.insert(Point { x: 1, y: 1 });
        assert_eq!(count_vertices(&inside), 4);

        // A rectangle of area 2
        inside.insert(Point { x: 1, y: 0 });
        assert_eq!(count_vertices(&inside), 4);

        // An L shape of area 3
        inside.insert(Point { x: 0, y: 1 });
        assert_eq!(count_vertices(&inside), 6);

        // A kind of S shape thingy?
        inside.insert(Point { x: 2, y: 0 });
        assert_eq!(count_vertices(&inside), 8);

        // A 3x3 box with a hole in the middle
        let mut inside = HashSet::new();
        inside.insert(Point { x: 0, y: 0 });
        inside.insert(Point { x: 1, y: 0 });
        inside.insert(Point { x: 2, y: 0 });
        inside.insert(Point { x: 0, y: 1 });
        inside.insert(Point { x: 2, y: 1 });
        inside.insert(Point { x: 0, y: 2 });
        inside.insert(Point { x: 1, y: 2 });
        inside.insert(Point { x: 2, y: 2 });
        assert_eq!(count_vertices(&inside), 8);
    }
}
