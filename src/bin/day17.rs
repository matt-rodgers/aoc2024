use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/17.in");
    let start = Instant::now();
    let (pt1, pt2) = run(&input);
    let elapsed = Instant::now() - start;
    println!("pt1: {} , pt2: {} , elapsed time {:?}", pt1, pt2, elapsed);
}

#[derive(Debug, Clone)]
struct Computer {
    rega: u32,
    regb: u32,
    regc: u32,
    program: Vec<u32>,
    pc: usize,
    output: Vec<u32>,
}

impl From<&str> for Computer {
    fn from(s: &str) -> Self {
        let mut parts = s.trim().lines();
        let (_, a) = parts.next().unwrap().split_at(12);
        let (_, b) = parts.next().unwrap().split_at(12);
        let (_, c) = parts.next().unwrap().split_at(12);
        let _ = parts.next().unwrap();
        let prg = parts.next().unwrap().trim_start_matches("Program: ");

        let rega = a.parse().unwrap();
        let regb = b.parse().unwrap();
        let regc = c.parse().unwrap();
        let program = prg.split(',').map(|n| n.parse().unwrap()).collect();

        Computer {
            rega,
            regb,
            regc,
            program,
            pc: 0,
            output: Vec::new(),
        }
    }
}

impl Computer {
    fn combo_operand(&self, operand: u32) -> Option<u32> {
        match operand {
            0..=3 => Some(operand),
            4 => Some(self.rega),
            5 => Some(self.regb),
            6 => Some(self.regc),
            _ => None,
        }
    }

    fn instruction(&mut self, opcode: u32, operand: u32) {
        let co = self.combo_operand(operand); // In case it's needed later

        let inc_ip = match opcode {
            0 | 6 | 7 => {
                let denominator = 2u32.pow(co.expect("Invalid combo operand"));
                let res = self.rega / denominator;
                match opcode {
                    0 => self.rega = res,
                    6 => self.regb = res,
                    7 => self.regc = res,
                    _ => unreachable!(),
                }
                true
            }
            1 => {
                self.regb = self.regb ^ operand;
                true
            }
            2 => {
                self.regb = co.expect("Invalid combo operand") % 8;
                true
            }
            3 => match self.rega {
                0 => true,
                _ => {
                    self.pc = operand as usize;
                    false
                }
            },
            4 => {
                self.regb = self.regb ^ self.regc;
                true
            }
            5 => {
                self.output.push(co.expect("Invalid combo operand") % 8);
                true
            }
            _ => {
                panic!("Invalid opcode {}", opcode);
            }
        };

        if inc_ip {
            self.pc += 2;
        }
    }

    fn run_until_halt(&mut self) -> String {
        loop {
            let instruction = self.program.get(self.pc);
            let operand = self.program.get(self.pc + 1);

            if instruction.is_none() || operand.is_none() {
                break;
            }

            self.instruction(*instruction.unwrap(), *operand.unwrap());
        }

        self.output.iter().join(",").to_string()
    }
}

fn run(input: &str) -> (String, u64) {
    let mut computer = Computer::from(input);
    let pt1 = computer.run_until_halt();

    (pt1, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../../inputs/17.ex");
        let (pt1, pt2) = run(&input);
        assert_eq!(pt1, "4,6,3,5,6,3,5,2,1,0".to_string());
        assert_eq!(pt2, 0);
    }
}
