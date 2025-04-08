use std::{collections::HashSet, fmt::Write, time::Instant};

pub fn run_outer() -> String {
    let input = include_str!("../inputs/04.in");
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

#[derive(Debug)]
struct CharGrid {
    xsize: usize,
    ysize: usize,
    grid: Vec<Vec<char>>,
}

impl std::fmt::Display for CharGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            for c in line.iter() {
                f.write_char(*c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct CharGridError;

impl TryFrom<&str> for CharGrid {
    type Error = CharGridError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<char>> = value
            .trim_end()
            .lines()
            .map(|line| line.trim_end().chars().collect())
            .collect();

        let xsize = grid.first().unwrap().len();
        if !grid.iter().skip(1).all(|line| line.len() == xsize) {
            return Err(CharGridError);
        }

        Ok(Self {
            xsize,
            ysize: grid.len(),
            grid,
        })
    }
}

impl CharGrid {
    fn lines_fwd(&self) -> Vec<String> {
        self.grid.iter().map(|line| line.iter().collect()).collect()
    }

    fn lines_rev(&self) -> Vec<String> {
        self.grid
            .iter()
            .map(|line| line.iter().rev().collect())
            .collect()
    }

    fn columns_fwd(&self) -> Vec<String> {
        let mut cols = Vec::with_capacity(self.ysize);

        for x in 0..self.xsize {
            let col: String = (0..self.ysize).map(|y| self.grid[y][x]).collect();
            cols.push(col);
        }

        cols
    }

    fn columns_rev(&self) -> Vec<String> {
        let mut cols = Vec::with_capacity(self.ysize);

        for x in 0..self.xsize {
            let col: String = (0..self.ysize).rev().map(|y| self.grid[y][x]).collect();
            cols.push(col);
        }

        cols
    }

    fn diagonals(&self, ydir: isize, xdir: isize) -> Vec<String> {
        let outer_count = self.ysize + self.xsize - 1;
        let inner_count = self.ysize.max(self.xsize);

        let mut diagonals = Vec::with_capacity(outer_count);
        let mut start_coords: HashSet<(usize, usize)> = HashSet::with_capacity(outer_count);

        for y in 0..self.ysize {
            let x = if xdir > 0 { 0 } else { self.xsize - 1 };
            start_coords.insert((x, y));
        }

        for x in 0..self.xsize {
            let y = if ydir > 0 { 0 } else { self.ysize - 1 };
            start_coords.insert((x, y));
        }

        for (mut xi, mut yi) in start_coords.iter() {
            let mut s = String::with_capacity(inner_count);

            while xi < self.xsize && yi < self.ysize {
                s.push(self.grid[yi][xi]);
                xi = xi.wrapping_add_signed(xdir);
                yi = yi.wrapping_add_signed(ydir);
            }

            diagonals.push(s);
        }

        diagonals
    }

    fn diagonals_rd(&self) -> Vec<String> {
        self.diagonals(1, 1)
    }

    fn diagonals_ld(&self) -> Vec<String> {
        self.diagonals(1, -1)
    }

    fn diagonals_ru(&self) -> Vec<String> {
        self.diagonals(-1, 1)
    }

    fn diagonals_lu(&self) -> Vec<String> {
        self.diagonals(-1, -1)
    }

    fn all_directions(&self) -> impl IntoIterator<Item = String> {
        let vecs = [
            self.lines_fwd().into_iter(),
            self.lines_rev().into_iter(),
            self.columns_fwd().into_iter(),
            self.columns_rev().into_iter(),
            self.diagonals_rd().into_iter(),
            self.diagonals_ru().into_iter(),
            self.diagonals_ld().into_iter(),
            self.diagonals_lu().into_iter(),
        ];

        vecs.into_iter().flatten()
    }

    fn count_xmas_pt2(&self) -> u64 {
        let mut count: u64 = 0;

        for y in (0..self.ysize).collect::<Vec<usize>>().windows(3) {
            for x in (0..self.xsize).collect::<Vec<usize>>().windows(3) {
                let sq: Vec<char> = y
                    .iter()
                    .flat_map(|yi| x.iter().map(move |xi| self.grid[*yi][*xi]))
                    .collect();

                // The possible XMAS's are:
                //
                //   M.S   S.M   S.S   M.M
                //   .A.   .A.   .A.   .A.
                //   M.S   S.M   M.M   S.S
                //
                // Positions 1, 3, 5, 7 are always irrelevant so skip them
                // Check all other positions against the patterns above

                match (sq[0], sq[2], sq[4], sq[6], sq[8]) {
                    ('M', 'S', 'A', 'M', 'S') => count += 1,
                    ('S', 'M', 'A', 'S', 'M') => count += 1,
                    ('S', 'S', 'A', 'M', 'M') => count += 1,
                    ('M', 'M', 'A', 'S', 'S') => count += 1,
                    _ => {}
                }
            }
        }

        count
    }
}

fn run(input: &str) -> (u64, u64) {
    let cg = CharGrid::try_from(input).unwrap();

    let pt1: u64 = cg
        .all_directions()
        .into_iter()
        .map(|s| s.matches("XMAS").count() as u64)
        .sum();

    let pt2 = cg.count_xmas_pt2();

    (pt1, pt2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../inputs/04.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, 18);
        assert_eq!(pt2, 9);
    }

    fn confirm_match(mut actual: Vec<String>, mut expected: Vec<&str>) {
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_grid() {
        // test grid laid out a bit nicer is:
        //
        //   a b c
        //   d e f
        //   g h i
        //
        let s = "abc\ndef\nghi\n";
        let cg = CharGrid::try_from(s).unwrap();

        confirm_match(cg.lines_fwd(), vec!["abc", "def", "ghi"]);
        confirm_match(cg.lines_rev(), vec!["cba", "fed", "ihg"]);
        confirm_match(cg.columns_fwd(), vec!["adg", "beh", "cfi"]);
        confirm_match(cg.columns_rev(), vec!["gda", "heb", "ifc"]);
        confirm_match(cg.diagonals_rd(), vec!["g", "dh", "aei", "bf", "c"]);
        confirm_match(cg.diagonals_ld(), vec!["a", "bd", "ceg", "fh", "i"]);
        confirm_match(cg.diagonals_ru(), vec!["i", "hf", "gec", "db", "a"]);
        confirm_match(cg.diagonals_lu(), vec!["g", "hd", "iea", "fb", "c"]);
    }

    #[test]
    fn test_grid_nonsquare() {
        // non-square test grid:
        //
        //   a b
        //   c d
        //   e f
        //
        let s = "ab\ncd\nef\n";
        let cg = CharGrid::try_from(s).unwrap();

        confirm_match(cg.lines_fwd(), vec!["ab", "cd", "ef"]);
        confirm_match(cg.lines_rev(), vec!["ba", "dc", "fe"]);
        confirm_match(cg.columns_fwd(), vec!["ace", "bdf"]);
        confirm_match(cg.columns_rev(), vec!["eca", "fdb"]);
        confirm_match(cg.diagonals_rd(), vec!["e", "cf", "ad", "b"]);
        confirm_match(cg.diagonals_ld(), vec!["a", "bc", "de", "f"]);
        confirm_match(cg.diagonals_ru(), vec!["f", "ed", "cb", "a"]);
        confirm_match(cg.diagonals_lu(), vec!["e", "fc", "da", "b"]);
    }

    #[test]
    fn test_all_directions() {
        // test grid:
        //
        //   a b
        //   c d
        //
        let s = "ab\ncd\n";
        let cg = CharGrid::try_from(s).unwrap();

        let all_strings: Vec<String> = cg.all_directions().into_iter().collect();
        confirm_match(
            all_strings,
            vec![
                "ab", "cd", "ba", "dc", "ac", "bd", "ca", "db", "c", "ad", "b", "a", "cb", "d",
                "a", "bc", "d", "c", "da", "b",
            ],
        );
    }
}
