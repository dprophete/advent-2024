use std::ops::BitXor;

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug)]
struct Machine {
    a: i32,
    b: i32,
    c: i32,
    prg: Vec<i32>,
    pc: usize,
    out: Vec<i32>,
}

impl Machine {
    pub fn from_str(input: &str) -> Machine {
        let mut lines = input.lines();
        let line_a = lines.next().unwrap();
        let a = toi32(line_a.split_once(": ").unwrap().1);
        let line_b = lines.next().unwrap();
        let b = toi32(line_b.split_once(": ").unwrap().1);
        let line_c = lines.next().unwrap();
        let c = toi32(line_c.split_once(": ").unwrap().1);
        lines.next(); // empty line
        let line_prg = lines.next().unwrap();
        let prg = line_prg
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(toi32)
            .collect();
        Machine {
            a,
            b,
            c,
            prg,
            pc: 0,
            out: vec![],
        }
    }

    pub fn combo(&self, operand: i32) -> i32 {
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
                self.a = self.a / 2_i32.pow(combo_v as u32);
                self.pc += 2;
            }
            1 => {
                // bxl
                self.b = self.b.bitxor(lit_v);
                self.pc += 2;
            }
            2 => {
                // bst
                self.b = combo_v % 8;
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
                self.out.push(combo_v % 8);
                self.pc += 2;
            }
            6 => {
                // bdv
                self.b = self.a / 2_i32.pow(combo_v as u32);
                self.pc += 2;
            }
            7 => {
                // cdv
                self.c = self.a / 2_i32.pow(combo_v as u32);
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

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day17: Warehouse Woes");
    time_it(p1, "p1", "data/17_sample.txt");
    time_it(p1, "p1", "data/17_input.txt");
    // time_it(p2, "p2", "data/17_sample.txt");
    // time_it(p2, "p2", "data/17_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(run_it(p1, "data/17_sample.txt"), 10092);
        // assert_eq!(run_it(p1, "data/17_sample_small.txt"), 2028);
        // assert_eq!(run_it(p2, "data/17_sample.txt"), 9021);
    }
}
