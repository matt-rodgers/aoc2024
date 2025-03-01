use num::Integer;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/13.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

const TOKENS_A: u64 = 3;
const TOKENS_B: u64 = 1;
const FLOAT_EQUAL_MARGIN: f64 = 0.000_001;

type Xy = (isize, isize);

#[derive(Debug, Clone, Default)]
struct Machine {
    a: Xy,
    b: Xy,
    prize: Xy,
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let mut it = s.lines();

        Machine {
            a: line_to_xy(it.next().unwrap(), 12, ", Y+"),
            b: line_to_xy(it.next().unwrap(), 12, ", Y+"),
            prize: line_to_xy(it.next().unwrap(), 9, ", Y="),
        }
    }
}

impl Machine {
    fn min_cost(&self) -> Option<u64> {
        // First check if a and b have the same ratio of x:y. For example if they're both 1, 2 or
        // a = 1, 2 and b = 2, 4 or something like that. In that case there is more than one
        // solution for how the prize number can be made out of a and b.
        if self.a.0 * self.b.1 == self.a.1 * self.b.0 {
            // First try to make the prize from only b (as this is the cheapest way).
            // If that doesn't work, incrementally add an a each time and try again.
            let mut a_count = 0;
            loop {
                let p = (
                    self.prize.0 - self.a.0 * a_count,
                    self.prize.1 - self.a.1 * a_count,
                );
                let (quotient, remainder) = p.0.div_rem(&self.b.0);
                if remainder == 0 && self.b.1 * quotient == p.1 {
                    // We can get to the prize
                    return Some(quotient as u64 * TOKENS_B + a_count as u64 * TOKENS_A);
                }

                if p.0 < self.a.0 || p.1 < self.a.1 {
                    // Impossible to get to the prize
                    return None;
                }

                a_count += 1;
            }
        }

        //
        // (1): xp = xa * n + xb * m
        // (2): yp = ya * n + yb * m
        //
        // We know xp, xp, xb, yp, ya, yb so solve for n and m
        //
        // rearrange (1): (xp - xb * m) / xa = n
        // sub into (2):  yp = ya * (xp - xb * m) / xa + yb * m
        //                yp = ya * xp / xa - ya * xb * m / xa + yb * m
        //                yp - ya * xp / xa = m * (yb - ya * xb / xa)
        //                m = (yp - ya * xp / xa) / (yb - ya * xb / xa)
        //
        // Since the equations above involve division it's a pain to use integer math.
        // So instead give it a go with f64, and confirm result is right using integer math
        let xp = self.prize.0 as f64;
        let yp = self.prize.1 as f64;
        let xa = self.a.0 as f64;
        let ya = self.a.1 as f64;
        let xb = self.b.0 as f64;
        let yb = self.b.1 as f64;

        let m = (yp - ya * xp / xa) / (yb - ya * xb / xa);
        if !float_approx_integer(m) {
            return None;
        }

        let n = (xp - xb * m) / xa;
        if !float_approx_integer(n) {
            return None;
        }

        let m = m.round() as isize;
        let n = n.round() as isize;
        if m < 0 || n < 0 {
            return None;
        }

        if self.prize.0 != self.a.0 * n + self.b.0 * m
            || self.prize.1 != self.a.1 * n + self.b.1 * m
        {
            return None;
        }

        return Some(TOKENS_B * m as u64 + TOKENS_A * n as u64);
    }
}

fn line_to_xy(line: &str, offset: usize, separator: &str) -> Xy {
    let (_, xy) = line.split_at(offset);
    let (x, y) = xy.split_once(separator).unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn float_approx_integer(n: f64) -> bool {
    if (n - n.round()).abs() < FLOAT_EQUAL_MARGIN {
        return true;
    }

    false
}

fn run(input: &str) -> (u64, u64) {
    let machines: Vec<Machine> = input
        .trim()
        .split("\n\n")
        .map(|m| Machine::from(m))
        .collect();

    let pt1: u64 = machines.iter().filter_map(|m| m.min_cost()).sum();

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/13.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 480);
        assert_eq!(pt2, 0);
    }
}
