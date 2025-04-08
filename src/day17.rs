use itertools::Itertools;
use std::time::Instant;

pub fn run_outer() -> String {
    let input = include_str!("../inputs/17.in");
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

fn run_simplified_computer(mut a: u32) -> String {
    let mut out: Vec<u32> = Vec::new();

    while a > 0 {
        let res = single_iteration(a & 0b1111111111);
        out.push(res);
        a = a >> 3;
    }

    out.iter().join(",").to_string()
}

fn single_iteration(a_lsbs: u32) -> u32 {
    let b = (a_lsbs & 0b111) ^ 0b111;
    let c = a_lsbs >> b;
    return ((b ^ 0b111) ^ c) & 0b111;
}

fn run(input: &str) -> (String, u64) {
    // Manual analysis of program:
    // 2, 4 - b = a % 8 # Put only the last 3 bits of a into b
    // 1, 7 - b = b ^ 7 # XOR b with 0b111, effectively flipping the 3 bits we just put in there
    // 7, 5 - c = a / 2.pow(b)
    // 0, 3 - a = a / 2.pow(3) # Divide a by 8, removing the last 3 bits
    // 1, 7 - b = b ^ 7 # XOR b with 0b111, flipping the bits back to the value before last XOR
    // 4, 1 - b = b ^ c
    // 5, 5 - push last 3 bits of b into output
    // 3, 0 - if a is zero halt, else go back to start
    //
    // We can rewrite and re-order this slightly:
    //   b   = (a & 0b111) ^ 0b111
    //   c   = a >> b
    //   out = ((b ^ 0b111) ^ c) & 0b111
    //   a   = a >> 3
    //   branch to start if a != 0
    //   end
    //
    // Each iteration:
    //   - depends only on the 10 least significant bits of a
    //   - outputs a single digit
    //   - consumes the 3 least significant bits of a
    //
    // For each output digit, there can only be a limited set of 10-bit values that will result in
    // this output. So we should just be able find the mapping of 10-bit input --> 3-bit output, and
    // then try all the possibilities to find the minimum one that works.

    // First confirm that our simplified equation works:
    let pt1 = run_simplified_computer(64012472);

    // Populate lookup table of output for each 10-bit input
    let mut lut = [0u8; 2usize.pow(10)];
    for (i, val) in lut.iter_mut().enumerate() {
        *val = single_iteration(i as u32).try_into().unwrap();
    }

    // Get the inverse of the lookup table - for each 3-bit output value, a Vec of the 10-bit inputs
    // that could create this output.
    let rev_lu: Vec<Vec<u32>> = (0..8)
        .map(|out| {
            lut.iter()
                .enumerate()
                .filter_map(|(i, val)| if *val == out { Some(i as u32) } else { None })
                .collect()
        })
        .collect();

    let (_, last_line) = input.trim().split_once("\n\n").unwrap();
    let (_, nums) = last_line.split_once(" ").unwrap();
    let program: Vec<u32> = nums.split(',').map(|n| n.parse().unwrap()).collect();

    let mut possible_answers: Vec<u64> = Vec::new();
    for (i, out) in program.iter().enumerate() {
        // Get all of the possible 10-bit inputs that could create this output
        let inputs = rev_lu
            .get(*out as usize)
            .expect("output does not exist in LUT...");

        // Insert into the possible answers any possible combination of this input and the possible
        // answers that aready exist. A combination is possible if the least significant 7 bits of
        // this input match the most significant 7 bits of a possible answer already in the set.
        let mut new_possible_answers = Vec::new();
        for ip in inputs.iter() {
            if i == 0 {
                // On the first iteration there are no existing possible answers, so all are valid
                new_possible_answers.push(*ip as u64);
            } else {
                for pa in possible_answers.iter() {
                    if *ip as u64 & 0b1111111 == (pa >> (i * 3)) & 0b1111111 {
                        new_possible_answers.push(pa | ((*ip as u64) << (i * 3)));
                    }
                }
            }
        }

        possible_answers = new_possible_answers;
    }

    let pt2 = possible_answers.iter().min().unwrap();

    (pt1, *pt2)
}

#[cfg(test)]
mod test {}
