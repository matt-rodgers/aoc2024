use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/07.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum AllowedOperators {
    Add,
    Multiply,
}

impl AllowedOperators {
    fn permutations_of_len(len: usize) -> Vec<Vec<Self>> {
        if len == 1 {
            return vec![vec![Self::Add], vec![Self::Multiply]];
        }

        let mut v = Vec::new();
        for perm in Self::permutations_of_len(len - 1) {
            let mut va = perm.clone();
            va.push(AllowedOperators::Add);
            v.push(va);
            let mut vm = perm.clone();
            vm.push(AllowedOperators::Multiply);
            v.push(vm);
        }

        v
    }
}

#[derive(Debug)]
struct Equation {
    testval: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let cpos = s.find(':').unwrap();
        let testval: u64 = s[0..cpos].parse().unwrap();
        let numbers: Vec<u64> = s[(cpos + 1)..]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Equation { testval, numbers }
    }
}

impl Equation {
    fn is_solvable(&self) -> bool {
        let n = self.numbers.len();

        for perm in AllowedOperators::permutations_of_len(n - 1) {
            let mut running_val = self.numbers[0];

            for (i, op) in perm.iter().enumerate() {
                match op {
                    AllowedOperators::Add => running_val = running_val + self.numbers[i + 1],
                    AllowedOperators::Multiply => running_val = running_val * self.numbers[i + 1],
                }
            }

            if running_val == self.testval {
                return true;
            }
        }

        false
    }
}

fn run(input: &str) -> (u64, u64) {
    let equations: Vec<Equation> = input
        .trim_end()
        .lines()
        .map(|line| Equation::from(line))
        .collect();

    let pt1: u64 = equations
        .iter()
        .filter_map(|equation| match equation.is_solvable() {
            true => Some(equation.testval),
            false => None,
        })
        .sum();

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/07.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 3749);
        assert_eq!(pt2, 0);
    }

    #[test]
    fn test_solvable() {
        let equation = Equation {
            testval: 190,
            numbers: vec![10, 19],
        };
        assert!(equation.is_solvable());

        let equation = Equation {
            testval: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(equation.is_solvable());

        let equation = Equation {
            testval: 161011,
            numbers: vec![16, 10, 13],
        };
        assert!(!equation.is_solvable());
    }

    #[test]
    fn test_permutations() {
        let mut v = AllowedOperators::permutations_of_len(1);
        v.sort();
        let mut expected = vec![
            vec![AllowedOperators::Add],
            vec![AllowedOperators::Multiply],
        ];
        expected.sort();
        assert_eq!(v, expected);

        let mut v = AllowedOperators::permutations_of_len(2);
        v.sort();
        let mut expected = vec![
            vec![AllowedOperators::Add, AllowedOperators::Add],
            vec![AllowedOperators::Add, AllowedOperators::Multiply],
            vec![AllowedOperators::Multiply, AllowedOperators::Multiply],
            vec![AllowedOperators::Multiply, AllowedOperators::Add],
        ];
        expected.sort();
        assert_eq!(v, expected);

        let mut v = AllowedOperators::permutations_of_len(3);
        v.sort();
        let mut expected = vec![
            vec![
                AllowedOperators::Add,
                AllowedOperators::Add,
                AllowedOperators::Add,
            ],
            vec![
                AllowedOperators::Add,
                AllowedOperators::Add,
                AllowedOperators::Multiply,
            ],
            vec![
                AllowedOperators::Add,
                AllowedOperators::Multiply,
                AllowedOperators::Add,
            ],
            vec![
                AllowedOperators::Add,
                AllowedOperators::Multiply,
                AllowedOperators::Multiply,
            ],
            vec![
                AllowedOperators::Multiply,
                AllowedOperators::Multiply,
                AllowedOperators::Add,
            ],
            vec![
                AllowedOperators::Multiply,
                AllowedOperators::Multiply,
                AllowedOperators::Multiply,
            ],
            vec![
                AllowedOperators::Multiply,
                AllowedOperators::Add,
                AllowedOperators::Add,
            ],
            vec![
                AllowedOperators::Multiply,
                AllowedOperators::Add,
                AllowedOperators::Multiply,
            ],
        ];
        expected.sort();
        assert_eq!(v, expected);
    }
}
