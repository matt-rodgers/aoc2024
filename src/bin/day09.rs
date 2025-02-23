use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/09.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug)]
struct File {
    id: u32,
    size: u32,
}

#[derive(Debug, Copy, Clone)]
enum CompactionState {
    FromFront,
    FromBack,
}

fn compact(mut files: Vec<File>, free_space: Vec<u32>) -> Vec<u32> {
    let count = files.iter().map(|f| f.size as usize).sum();
    let mut out = Vec::with_capacity(count);

    let mut state = CompactionState::FromFront;
    let mut fwd_idx = 0;
    let mut back_idx = files.len() - 1;
    let mut free_space_idx = 0;

    'outer: while out.len() < count {
        match state {
            CompactionState::FromFront => {
                let f = &mut files[fwd_idx];
                for _ in 0..f.size {
                    out.push(f.id);
                }
                fwd_idx += 1;
                f.size = 0;
                state = CompactionState::FromBack;
            }
            CompactionState::FromBack => {
                let fs = free_space[free_space_idx];
                for _ in 0..fs {
                    let mut f = &mut files[back_idx];

                    while f.size == 0 {
                        if back_idx > 0 {
                            back_idx -= 1;
                        } else {
                            break 'outer;
                        }
                        f = &mut files[back_idx];
                    }

                    out.push(f.id);
                    f.size -= 1;
                }

                free_space_idx += 1;
                state = CompactionState::FromFront;
            }
        }
    }

    out
}

fn run(input: &str) -> (u64, u64) {
    let ic = input.trim().chars();

    let files: Vec<File> = ic
        .clone()
        .step_by(2)
        .enumerate()
        .map(|(i, c)| File {
            id: i as u32,
            size: c.to_digit(10).unwrap(),
        })
        .collect();

    let free_space: Vec<u32> = ic
        .skip(1)
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let compacted = compact(files, free_space);
    let pt1: u64 = compacted
        .iter()
        .enumerate()
        .map(|(i, n)| i as u64 * *n as u64)
        .sum();

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/09.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 1928);
        assert_eq!(pt2, 0);
    }

    #[test]
    fn test_compact() {
        let files = VecDeque::from(vec![
            File { id: 0, size: 1 },
            File { id: 1, size: 3 },
            File { id: 2, size: 5 },
        ]);
        let free_space = vec![2, 4];
        let res = compact(files, free_space);
        assert_eq!(res, vec![0, 2, 2, 1, 1, 1, 2, 2, 2]);
    }
}
