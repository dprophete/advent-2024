use std::ops::BitXor;

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    prg: Vec<u64>,
    pc: usize,
    out: Vec<u64>,
}

impl Machine {
    pub fn from_str(input: &str) -> Machine {
        let mut lines = input.lines();
        let line_a = lines.next().unwrap();
        let a = tou64(line_a.split_once(": ").unwrap().1);
        let line_b = lines.next().unwrap();
        let b = tou64(line_b.split_once(": ").unwrap().1);
        let line_c = lines.next().unwrap();
        let c = tou64(line_c.split_once(": ").unwrap().1);
        lines.next(); // empty line
        let line_prg = lines.next().unwrap();
        let prg = line_prg
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(tou64)
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

    pub fn combo(&self, operand: u64) -> u64 {
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
                self.a >>= combo_v;
                self.pc += 2;
                // println!("adv: a={}", self.a);
            }
            1 => {
                // bxl
                self.b = self.b.bitxor(lit_v);
                self.pc += 2;
                // println!("bxl: b={}", self.b);
            }
            2 => {
                // bst
                self.b = combo_v & 7;
                self.pc += 2;
                // println!("bst: b={}", self.b);
            }
            3 => {
                // jnz
                if self.a == 0 {
                    // println!("jnz: noop");
                    self.pc += 2;
                } else {
                    // println!("jnz: pc={}", lit_v);
                    self.pc = lit_v as usize;
                }
            }
            4 => {
                // bxc
                self.b = self.b.bitxor(self.c);
                self.pc += 2;
                // println!("bxc: b={}", self.b);
            }
            5 => {
                // out
                self.out.push(combo_v & 7);
                self.pc += 2;
                // println!("out: val={}", combo_v & 7);
            }
            6 => {
                // bdv
                self.b = self.a >> combo_v;
                self.pc += 2;
                // println!("bdv: b={}", self.b);
            }
            7 => {
                // cdv
                self.c = self.a >> combo_v;
                self.pc += 2;
                // println!("cdv: c={}", self.c);
            }
            _ => panic!("invalid opcode"),
        }
    }

    pub fn run_prg(&mut self) {
        while self.pc < self.prg.len() {
            self.run_at_pc();
        }
    }

    pub fn run_with_a(&self, a: u64) -> Vec<u64> {
        let mut m = self.clone();
        m.a = a;
        m.run_prg();
        m.out
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

// observations:
// =============
// - length of count:
//     0 to 2^3-1 -> 1 out
//     2^3 to 2^6-1 -> 2 out
//     2^6 to 2^9-1 -> 3 out
//     2^9 to 2^12-1 -> 4 out
//     2^12  -> 5 out
//     2^15 -> 6 out
//     2^18  -> 7 out
//     2^21  -> 8 out
//     2^24  -> 9 out
//     2^27 -> 10 out
//     2^30 -> 11 out
// - in an internval where out is of length L (so 2^(L*3) to 2^(L*3+3)-1)
//      - last out entry is always going through the same series: 4, 6, 7, 0, 1, 2, 3, repeated 2^(L*3) times each
//      - previous out entry goes thought a sequence as well, each repeated 2^(L*3-3)
//      - then previous out goes though a sequence of 2^(L*3-6)
//      - so for L = 5: from 4096 to 32767
//          - last digit changes every  4096 times
//          - previous digit changes every 512 times (ratio 2^3)
//          - previous digit changes every 64 times (ratio 2^3)
//          - previous digit changes every 8 times (ratio 2^3)

fn p2(input: &str) -> u64 {
    let machine = Machine::from_str(input);
    let mut solution = u64::MAX;

    let len = machine.prg.len();
    let idx = (len - 1) as i32;
    let power = idx * 3;

    let mut to_explore = vec![(1 << power, idx)];

    while let Some((mut start, idx)) = to_explore.pop() {
        if idx == -1 {
            if start < solution {
                solution = start;
            }
            continue;
        }
        let power = idx * 3;
        let offset = 1 << power;
        let digit_to_match = machine.prg[idx as usize];
        for _ in 0..8 {
            let out = machine.run_with_a(start);
            if out[idx as usize] == digit_to_match {
                to_explore.push((start, idx - 1));
            }
            start += offset;
        }
    }
    solution
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day17: Warehouse Woes");
    time_it(p1, "p1", "data/17_sample.txt");
    time_it(p1, "p1", "data/17_sample2.txt");
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
        assert_eq!(run_it(p2, "data/17_input.txt"), 164542125272765);
    }
}
