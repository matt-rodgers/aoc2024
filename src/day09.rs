use std::time::Instant;

pub fn run_outer() -> String {
    let input = include_str!("../inputs/09.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    format!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed)
}

#[derive(Debug, Clone)]
struct File {
    location: usize,
    id: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct FreeSpace {
    location: usize,
    size: usize,
}

#[derive(Debug, Copy, Clone)]
enum CompactionState {
    FromFront,
    FromBack,
}

fn compact(mut files: Vec<File>, free_space: Vec<FreeSpace>) -> Vec<usize> {
    let count = files.iter().map(|f| f.size).sum();
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
                let fs = &free_space[free_space_idx];
                for _ in 0..fs.size {
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

fn compact_pt2(files: Vec<File>, mut free_space: Vec<FreeSpace>) -> Vec<Option<usize>> {
    let count = files.iter().map(|f| f.size).sum::<usize>()
        + free_space.iter().map(|fs| fs.size).sum::<usize>();
    let mut out = vec![None; count];

    'outer: for (i, file) in files.iter().enumerate().rev() {
        // See if we can find a space to the left of the file that it will fit into
        let len = free_space.len();
        for fs in free_space[0..i.min(len)].iter_mut() {
            if file.size <= fs.size {
                // It fits, insert into output, update the free space, and break here
                for n in fs.location..fs.location + file.size {
                    out[n] = Some(file.id);
                }

                fs.size -= file.size;
                fs.location += file.size;

                continue 'outer;
            }
        }

        // If the file cannot be moved anywhere, insert into output where it currently is
        for n in file.location..file.location + file.size {
            out[n] = Some(file.id);
        }
    }

    out
}

fn run(input: &str) -> (u64, u64) {
    let mut files = Vec::<File>::new();
    let mut free_space = Vec::<FreeSpace>::new();
    let mut location = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            files.push(File {
                location,
                id: i / 2,
                size,
            });
        } else {
            free_space.push(FreeSpace { location, size });
        }

        location += size;
    }

    let compacted = compact(files.clone(), free_space.clone());
    let pt1 = compacted
        .iter()
        .enumerate()
        .map(|(i, n)| i * n)
        .sum::<usize>() as u64;

    let compacted_pt2 = compact_pt2(files, free_space);
    let pt2 = compacted_pt2
        .iter()
        .enumerate()
        .map(|(i, out)| if let Some(n) = out { i * n } else { 0 })
        .sum::<usize>() as u64;

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/09.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 1928);
        assert_eq!(pt2, 2858);
    }

    #[test]
    fn test_compact() {
        let files = vec![
            File {
                location: 0,
                id: 0,
                size: 1,
            },
            File {
                location: 3,
                id: 1,
                size: 3,
            },
            File {
                location: 10,
                id: 2,
                size: 5,
            },
        ];
        let free_space = vec![
            FreeSpace {
                location: 1,
                size: 2,
            },
            FreeSpace {
                location: 6,
                size: 4,
            },
        ];
        let res = compact(files, free_space);
        assert_eq!(res, vec![0, 2, 2, 1, 1, 1, 2, 2, 2]);
    }
}
