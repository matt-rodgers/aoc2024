use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;

pub fn run_outer() -> String {
    let input = include_str!("../inputs/24.in");
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

fn run(input: &str) -> (u64, String) {
    let (initial_values, connections) = input.trim().split_once("\n\n").unwrap();

    let original_inputs: HashMap<&str, bool> = initial_values
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            let n: u32 = val.parse().unwrap();
            (name, n > 0)
        })
        .collect();

    let original_connections: Vec<Connection> = connections
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

    let mut inputs = original_inputs.clone();
    let mut connections = original_connections.clone();

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

    let maxz = original_connections
        .iter()
        .filter_map(|val| extract_bit(val.output, 'z'))
        .max()
        .unwrap();

    // The system appears to be a ripple carry adder.
    //
    // See: https://www.ece.uvic.ca/~fayez/courses/ceng465/lab_465/project1/adders.pdf
    //
    // Based on this, we can identify a few possible types of connection that must be wrong:
    // 1. All 'z' outputs apart from the MSB must come from an XOR gate.
    // 2. The 'z' MSB output must come from an OR gate
    // 3. All XOR gates must be connected to the top level input or the final output (or both)
    // 4. The output of any AND gate must be connected to the input of an OR gate, with the
    //    exception of the AND gate with x00 and y00 inputs (since on the first stage the carry bit
    //    is always zero, and therefore the adder can be optimised by removing an AND and an OR gate)
    // 5. The output of an XOR gate must not be connected to the input of an OR gate

    let mut wrong: Vec<&str> = Vec::new();
    for conn in original_connections.iter() {
        if conn.output.starts_with('z') {
            if !conn.output.contains(&maxz.to_string()) {
                // 1.
                if conn.operation != Operation::Xor {
                    wrong.push(conn.output);
                    continue;
                }
            } else {
                // 2.
                if conn.operation != Operation::Or {
                    wrong.push(conn.output);
                    continue;
                }
            }
        }

        // 3.
        if conn.operation == Operation::Xor {
            if ![conn.output, conn.inputs.0, conn.inputs.1]
                .iter()
                .any(|val| val.starts_with('x') || val.starts_with('y') || val.starts_with('z'))
            {
                wrong.push(conn.output);
                continue;
            }
        }

        // 4.
        if conn.operation == Operation::And {
            // Exclude the very first AND gate (with inputs x00 and y00)
            if [conn.inputs.0, conn.inputs.1]
                .iter()
                .any(|val| val.contains("x00") || val.contains("y00"))
            {
                continue;
            }

            // Find all gates which have an input equal to this output, and check if OR gates
            for next_conn in original_connections.iter() {
                if next_conn.operation != Operation::Or
                    && (next_conn.inputs.0 == conn.output || next_conn.inputs.1 == conn.output)
                {
                    wrong.push(conn.output);
                    continue;
                }
            }
        }

        // 5.
        if conn.operation == Operation::Xor {
            for next_conn in original_connections.iter() {
                if next_conn.operation == Operation::Or
                    && (next_conn.inputs.0 == conn.output || next_conn.inputs.1 == conn.output)
                {
                    wrong.push(conn.output);
                    continue;
                }
            }
        }
    }

    wrong.sort();

    (pt1, wrong.iter().dedup().join(","))
}

fn extract_bit(ip: &str, prefix: char) -> Option<u32> {
    match ip.starts_with(prefix) {
        true => Some(ip[1..].parse().unwrap()),
        false => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/24.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 4);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../inputs/24_2.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 2024);
    }
}
