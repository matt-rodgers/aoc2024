use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../inputs/21.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn run(input: &str) -> (u64, u64) {
    let codes: Vec<(u64, Vec<char>)> = input
        .trim()
        .lines()
        .map(|line| (line[0..3].parse().unwrap(), line.chars().collect()))
        .collect();

    let mut pt1 = 0;
    let mut pt2 = 0;

    let mut cache = HashMap::new();

    for (n, chars) in codes {
        pt1 += run_one_sequence(n, &chars, 2, &mut cache);
        pt2 += run_one_sequence(n, &chars, 25, &mut cache);
    }

    (pt1, pt2)
}

fn run_one_sequence(
    n: u64,
    chars: &Vec<char>,
    directional_keypads: u32,
    cache: &mut HashMap<(Vec<char>, u32), u64>,
) -> u64 {
    let seq = seq_numeric(&chars);
    let l = directional_recurse(seq, directional_keypads, cache);
    l * n
}

fn numeric_pos(c: char) -> (isize, isize) {
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("invalid character"),
    }
}

fn directional_pos(c: char) -> (isize, isize) {
    match c {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("invalid character"),
    }
}

fn move_numeric(start: char, end: char) -> Vec<char> {
    let start_pos = numeric_pos(start);
    let end_pos = numeric_pos(end);
    let mut out = Vec::new();

    // It's always best to do either:
    // - all of the horizontal moves, then all of the vertical moves, or
    // - all of the vertical moves, then all of the horizontal moves
    // to maximise repeated button presses.
    //
    // If possible (ie without going over the blank square), we should prioritise moves in the
    // following order: < v ^ >
    //
    // The numeric keypad looks like this:
    //
    //   7 8 9
    //   4 5 6
    //   1 2 3
    //     0 A

    if start_pos.1 == 3 && end_pos.0 == 0 {
        // We have to go up first to avoid the blank square
        for _ in 0..(start_pos.1 - end_pos.1) {
            out.push('^');
        }

        for _ in 0..(start_pos.0 - end_pos.0) {
            out.push('<');
        }
    } else if start_pos.0 == 0 && end_pos.1 == 3 {
        // We have to go right first to avoid the blank square
        for _ in 0..(end_pos.0 - start_pos.0) {
            out.push('>');
        }

        for _ in 0..(end_pos.1 - start_pos.1) {
            out.push('v');
        }
    } else {
        // We are free to move in any direction first
        if start_pos.0 > end_pos.0 {
            for _ in 0..(start_pos.0 - end_pos.0) {
                out.push('<');
            }
        }

        if start_pos.1 < end_pos.1 {
            for _ in 0..(end_pos.1 - start_pos.1) {
                out.push('v');
            }
        }

        if start_pos.1 > end_pos.1 {
            for _ in 0..(start_pos.1 - end_pos.1) {
                out.push('^');
            }
        }

        if start_pos.0 < end_pos.0 {
            for _ in 0..(end_pos.0 - start_pos.0) {
                out.push('>');
            }
        }
    }

    out.push('A');
    out
}

fn move_directional(start: char, end: char) -> Vec<char> {
    let start_pos = directional_pos(start);
    let end_pos = directional_pos(end);
    let mut out = Vec::new();

    // It's always best to do either:
    // - all of the horizontal moves, then all of the vertical moves, or
    // - all of the vertical moves, then all of the horizontal moves
    // to maximise repeated button presses.
    //
    // If possible (ie without going over the blank square), we should prioritise moves in the
    // following order: < ^ v >
    //
    // The directional keypad looks like this:
    //
    //   ^ A
    // < v >

    if start_pos.0 == 0 && end_pos.1 == 0 {
        // We have to go right first to avoid the blank square
        for _ in 0..(end_pos.0 - start_pos.0) {
            out.push('>');
        }

        for _ in 0..(start_pos.1 - end_pos.1) {
            out.push('^');
        }
    } else if start_pos.1 == 0 && end_pos.0 == 0 {
        // We have to go down first to avoid the blank square
        for _ in 0..(end_pos.1 - start_pos.1) {
            out.push('v');
        }

        for _ in 0..(start_pos.0 - end_pos.0) {
            out.push('<');
        }
    } else {
        // We are free to move in any direction first
        if start_pos.0 > end_pos.0 {
            for _ in 0..(start_pos.0 - end_pos.0) {
                out.push('<');
            }
        }

        if start_pos.1 < end_pos.1 {
            for _ in 0..(end_pos.1 - start_pos.1) {
                out.push('v');
            }
        }

        if start_pos.1 > end_pos.1 {
            for _ in 0..(start_pos.1 - end_pos.1) {
                out.push('^');
            }
        }

        if start_pos.0 < end_pos.0 {
            for _ in 0..(end_pos.0 - start_pos.0) {
                out.push('>');
            }
        }
    }

    out.push('A');
    out
}

fn seq_numeric(ip: &[char]) -> Vec<char> {
    let mut current = 'A';
    let mut out = Vec::new();

    for c in ip.iter() {
        let next_seq = move_numeric(current, *c);
        out.extend_from_slice(&next_seq);
        current = *c;
    }

    out
}

fn directional_recurse(
    ip: Vec<char>,
    depth: u32,
    cache: &mut HashMap<(Vec<char>, u32), u64>,
) -> u64 {
    if let Some(res) = cache.get(&(ip.clone(), depth)) {
        return *res;
    }

    let mut current = 'A';
    let mut res = 0;

    for c in ip.iter() {
        let seq = move_directional(current, *c);

        if depth == 1 {
            res += seq.len() as u64;
        } else {
            res += directional_recurse(seq, depth - 1, cache);
        }

        current = *c;
    }

    cache.insert((ip, depth), res);

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/21.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 126384);
    }

    #[test]
    fn test_move_numeric() {
        assert_eq!(move_numeric('7', '9'), vec!['>', '>', 'A']);
        assert_eq!(move_numeric('6', '4'), vec!['<', '<', 'A']);
        assert_eq!(move_numeric('1', '7'), vec!['^', '^', 'A']);
        assert_eq!(move_numeric('8', '2'), vec!['v', 'v', 'A']);
        assert_eq!(move_numeric('7', '0'), vec!['>', 'v', 'v', 'v', 'A']);
        assert_eq!(move_numeric('A', '4'), vec!['^', '^', '<', '<', 'A']);
        assert_eq!(move_numeric('1', '6'), vec!['^', '>', '>', 'A']);
    }

    #[test]
    fn test_move_directional() {
        assert_eq!(move_directional('<', '>'), vec!['>', '>', 'A']);
        assert_eq!(move_directional('^', 'v'), vec!['v', 'A']);
        assert_eq!(move_directional('<', 'A'), vec!['>', '>', '^', 'A']);
        assert_eq!(move_directional('A', '<'), vec!['v', '<', '<', 'A']);
    }

    #[test]
    fn test_seq_numeric() {
        let input: Vec<char> = "029A".chars().collect();
        let seq = seq_numeric(&input);
        assert_eq!(seq.len(), "<A^A>^^AvvvA".len());
    }
}
