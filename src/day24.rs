use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    pub fn eval(&self, in1: bool, in2: bool) -> bool {
        match self {
            Op::And => in1 && in2,
            Op::Or => in1 || in2,
            Op::Xor => in1 ^ in2,
        }
    }

    pub fn from_str(s: &str) -> Op {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("invalid op {}", s),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Gate {
    in1: String, // input wire 1
    in2: String, // input wire 2
    out: String, // output wire
    op: Op,
}

impl Gate {
    pub fn eval(&self, wires: &HashMap<String, bool>) -> bool {
        let in1 = wires.get(&self.in1).unwrap();
        let in2 = wires.get(&self.in2).unwrap();
        self.op.eval(*in1, *in2)
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    wires: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let (wires_str, gates_str) = input.split_once("\n\n").unwrap();

        // wires
        let wires: HashMap<String, bool> = wires_str
            .lines()
            .map(|line| {
                let (w, v) = line.split_once(": ").unwrap();
                (String::from(w), v == "1")
            })
            .collect();

        // gates
        let gates = gates_str
            .lines()
            .map(|line| {
                let re = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)$").unwrap();
                let (_, [in1, op, in2, out]) = re.captures(line).unwrap().extract();
                Gate {
                    in1: String::from(in1),
                    in2: String::from(in2),
                    out: String::from(out),
                    op: Op::from_str(op),
                }
            })
            .collect();

        Puzzle { wires, gates }
    }

    pub fn eval(&mut self) {
        let mut gates_to_eval: HashSet<&Gate> = HashSet::new();
        for g in self.gates.iter() {
            gates_to_eval.insert(g);
        }

        // we are going to evaluate all the gates which have their wire ready
        // and then go back to the beginning until all gates are evaluated
        // note: this is not optimal at all...
        // ideally we build a proper graph of gate evaluation order
        while !gates_to_eval.is_empty() {
            let mut gates_to_remove = vec![];
            for &g in gates_to_eval.iter() {
                match (self.wires.get(&g.in1), self.wires.get(&g.in2)) {
                    (Some(&in1), Some(&in2)) => {
                        let gate_out = g.op.eval(in1, in2);
                        self.wires.insert(g.out.clone(), gate_out);
                        gates_to_remove.push(g);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            for g in gates_to_remove {
                gates_to_eval.remove(g);
            }
        }
    }

    fn wires_with_prefix(&self, prefix: char) -> String {
        let mut wires = self
            .wires
            .iter()
            .filter(|(k, _)| k.chars().nth(0).unwrap() == prefix)
            .map(|k| k.clone())
            .collect::<Vec<_>>();
        wires.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
        String::from_iter(wires.iter().map(|(_, &v)| if v { '1' } else { '0' }))
    }

    pub fn p1(&mut self) -> usize {
        self.eval();

        let z = self.wires_with_prefix('z');
        usize::from_str_radix(&z, 2).unwrap()
    }
}

fn p1(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);
    puzzle.p1()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> String {
//     let puzzle = Puzzle::from_str(input);
//     puzzle.p2()
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day24: Crossed Wires");
    time_it(p1, "p1", "data/24_sample.txt");
    time_it(p1, "p1", "data/24_sample2.txt");
    time_it(p1, "p1", "data/24_input.txt");
    // time_it(p2, "p2", "data/24_sample.txt");
    // time_it(p2, "p2", "data/24_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/24_sample.txt"), 4);
        assert_eq!(run_it(p1, "data/24_sample2.txt"), 2024);
        assert_eq!(run_it(p1, "data/24_input.txt"), 55544677167336);
        // assert_eq!(run_it(p2, "data/24_sample.txt"), "co,de,ka,ta");
        // assert_eq!(
        //     run_it(p2, "data/24_input.txt"),
        //     "bv,cm,dk,em,gs,jv,ml,oy,qj,ri,uo,xk,yw"
        // );
    }
}
