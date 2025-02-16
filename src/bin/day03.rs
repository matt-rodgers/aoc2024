use regex::Regex;
use std::sync::OnceLock;
use std::time::Instant;

static REGEX: OnceLock<Regex> = OnceLock::new();

fn main() {
    let input = include_str!("../../inputs/03.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

fn sum_mul_instructions(ip: &str) -> u64 {
    // Lazy init of static regex (takes a while to compile regex so make sure to only do it once)
    let rgx = REGEX.get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

    rgx.captures_iter(ip)
        .map(|cap| {
            // Note: .get(0) returns the entire match. We just want the numbers.
            let capa = cap.get(1).unwrap().as_str();
            let capb = cap.get(2).unwrap().as_str();

            let a = u64::from_str_radix(capa, 10).unwrap();
            let b = u64::from_str_radix(capb, 10).unwrap();

            a * b
        })
        .sum()
}

fn run(input: &str) -> (u64, u64) {
    let pt1 = sum_mul_instructions(input);

    let mut do_indices = input.match_indices("do()").map(|(i, _)| i);
    let mut dont_indices = input.match_indices("don't()").map(|(i, _)| i);

    let mut enabled_sections = Vec::<&str>::new();
    let mut i: usize = 0;
    let mut enabled: bool = true;

    while i < input.len() {
        match enabled {
            true => {
                // If enabled, find the next don't() which is past the current index
                let mut dont_idx: Option<usize> = None;
                while let Some(n) = dont_indices.next() {
                    if n > i {
                        dont_idx = Some(n);
                        break;
                    }
                }

                if let Some(n) = dont_idx {
                    // If we found a suitable don't instruction, store the enabled section and continue
                    enabled_sections.push(&input[i..n]);
                    i = n;
                } else {
                    // If there is no suitable don't instruction, store the rest of the input and finish
                    enabled_sections.push(&input[i..]);
                    i = input.len();
                }

                enabled = false;
            }
            false => {
                // If disabled, find the next do() which is past the current index
                let mut do_idx: Option<usize> = None;
                while let Some(n) = do_indices.next() {
                    if n > i {
                        do_idx = Some(n);
                        break;
                    }
                }

                if let Some(n) = do_idx {
                    i = n;
                } else {
                    i = input.len();
                }

                enabled = true;
            }
        }
    }

    let pt2: u64 = enabled_sections
        .iter()
        .map(|s| sum_mul_instructions(s))
        .sum();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/03.ex");
        let (pt1, _pt2) = run(&input);
        assert_eq!(pt1, 161);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../../inputs/03_2.ex");
        let (_pt1, pt2) = run(&input);
        assert_eq!(pt2, 48);
    }
}
