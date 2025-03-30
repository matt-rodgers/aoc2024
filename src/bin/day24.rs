use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/24.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Connection<'a> {
    inputs: (&'a str, &'a str),
    operation: Operation,
    output: &'a str,
}

fn run(input: &str) -> (u64, u64) {
    let (initial_values, connections) = input.trim().split_once("\n\n").unwrap();

    let mut inputs: HashMap<&str, bool> = initial_values
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            let n: u32 = val.parse().unwrap();
            (name, n > 0)
        })
        .collect();

    let mut connections: Vec<Connection> = connections
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap();
            let operation = Operation::try_from(parts.next().unwrap()).unwrap();
            let b = parts.next().unwrap();
            let output = parts.skip(1).next().unwrap();
            Connection {
                inputs: (a, b),
                operation,
                output,
            }
        })
        .collect();

    while !connections.is_empty() {
        let mut new_connections: Vec<Connection> = Vec::new();
        let mut new_inputs = inputs.clone();
        for conn in connections {
            let a = inputs.get(conn.inputs.0);
            let b = inputs.get(conn.inputs.1);
            match (a, b) {
                (Some(a), Some(b)) => {
                    let res = match conn.operation {
                        Operation::And => *a && *b,
                        Operation::Or => *a || *b,
                        Operation::Xor => *a ^ *b,
                    };
                    new_inputs.insert(conn.output, res);
                }
                _ => {
                    new_connections.push(conn);
                }
            }
        }

        connections = new_connections;
        inputs = new_inputs;
    }

    let mut zs: Vec<(&str, bool)> = inputs
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with('z') {
                Some((*k, *v))
            } else {
                None
            }
        })
        .collect();

    zs.sort_by_key(|elem| elem.0);

    let mut pt1 = 0u64;
    assert!(zs.len() < 64);
    for (i, z) in zs.iter().enumerate() {
        if z.1 {
            pt1 |= 1u64 << i;
        }
    }

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/24.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 4);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../../inputs/24_2.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 2024);
    }
}
