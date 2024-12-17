use std::{ops::BitXor, time::Instant};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Machine {
    a: u32,
    b: u32,
    c: u32,
    prg: Vec<u32>,
    pc: usize,
    out: Vec<u32>,
    out_size: usize,
}

impl Machine {
    pub fn from_str(input: &str) -> Machine {
        let mut lines = input.lines();
        let line_a = lines.next().unwrap();
        let a = tou32(line_a.split_once(": ").unwrap().1);
        let line_b = lines.next().unwrap();
        let b = tou32(line_b.split_once(": ").unwrap().1);
        let line_c = lines.next().unwrap();
        let c = tou32(line_c.split_once(": ").unwrap().1);
        lines.next(); // empty line
        let line_prg = lines.next().unwrap();
        let prg = line_prg
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(tou32)
            .collect();
        Machine {
            a,
            b,
            c,
            prg,
            pc: 0,
            out: vec![],
            out_size: 0,
        }
    }

    pub fn combo(&self, operand: u32) -> u32 {
        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => operand,
        }
    }

    pub fn run_at_pc(&mut self) {
        let opcode = self.prg[self.pc];
        let operand = self.prg[self.pc + 1];
        let lit_v = operand;
        let combo_v = self.combo(operand);
        match opcode {
            0 => {
                // adv
                self.a = self.a >> combo_v;
                self.pc += 2;
            }
            1 => {
                // bxl
                self.b = self.b.bitxor(lit_v);
                self.pc += 2;
            }
            2 => {
                // bst
                self.b = combo_v & 7;
                self.pc += 2;
            }
            3 => {
                // jnz
                if self.a == 0 {
                    self.pc += 2;
                } else {
                    self.pc = lit_v as usize;
                }
            }
            4 => {
                // bxc
                self.b = self.b.bitxor(self.c);
                self.pc += 2;
            }
            5 => {
                // out
                self.out.push(combo_v & 7);
                self.out_size += 1;
                self.pc += 2;
            }
            6 => {
                // bdv
                self.b = self.a >> combo_v;
                self.pc += 2;
            }
            7 => {
                // cdv
                self.c = self.a >> combo_v;
                self.pc += 2;
            }
            _ => panic!("invalid opcode"),
        }
    }

    pub fn run_prg(&mut self) {
        while self.pc < self.prg.len() {
            self.run_at_pc();
        }
    }

    pub fn run_prg_with_limit(&mut self) -> bool {
        let prg_len = self.prg.len();
        let mut out_len = 0;
        while self.pc < prg_len {
            let was_out = self.prg[self.pc] == 5;
            self.run_at_pc();
            if was_out {
                out_len += 1;
                if out_len > prg_len {
                    return false;
                }
                if self.out.last() != Some(&self.prg[out_len - 1]) {
                    return false;
                }
            }
        }
        out_len == prg_len
    }
}

fn p1(input: &str) -> String {
    let mut machine = Machine::from_str(input);
    machine.run_prg();
    machine
        .out
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> u32 {
    let base_machine = Machine::from_str(input);

    let start = Instant::now();
    let mut a = 0;
    loop {
        if (a % 100_000_000) == 0 {
            println!("[{}] a={}", fmt_duration(start.elapsed()), a / 100_000_000);
        }
        let mut machine = base_machine.clone();
        machine.a = a;
        let same_out_len = machine.run_prg_with_limit();
        if same_out_len {
            if machine.out == machine.prg {
                break;
            }
        }
        a += 1;
    }
    a
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day17: Warehouse Woes");
    time_it(p1, "p1", "data/17_sample.txt");
    time_it(p1, "p1", "data/17_input.txt");
    time_it(p2, "p2", "data/17_sample2.txt");
    time_it(p2, "p2", "data/17_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/17_sample.txt"), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(run_it(p1, "data/17_input.txt"), "4,1,5,3,1,5,3,5,7");
        assert_eq!(run_it(p2, "data/17_sample2.txt"), 117440);
        // assert_eq!(run_it(p1, "data/17_sample_small.txt"), 2028);
        // assert_eq!(run_it(p2, "data/17_sample.txt"), 9021);
    }
}
