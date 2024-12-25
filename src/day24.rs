use core::fmt;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

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

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Op::And => "AND",
            Op::Or => "OR",
            Op::Xor => "XOR",
        };
        write!(f, "{}", s)?;
        Ok(())
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

    // pub fn eval_wire(&mut self) {
    //     if self.wires.contains_key("a") {
    //         return;
    //     }
    // }
    //
    // pub fn eval2(&mut self) {
    //     let z_wires = self
    //         .wires
    //         .keys()
    //         .filter(|(k)| k.chars().nth(0).unwrap() == 'z')
    //         .collect::<Vec<_>>();
    //
    //
    //     let mut gates_to_eval: HashSet<&Gate> = HashSet::new();
    //     for g in self.gates.iter() {
    //         gates_to_eval.insert(g);
    //     }
    //
    //     // we are going to evaluate all the gates which have their wire ready
    //     // and then go back to the beginning until all gates are evaluated
    //     // note: this is not optimal at all...
    //     // ideally we build a proper graph of gate evaluation order
    //     while !gates_to_eval.is_empty() {
    //         let mut gates_to_remove = vec![];
    //         for &g in gates_to_eval.iter() {
    //             match (self.wires.get(&g.in1), self.wires.get(&g.in2)) {
    //                 (Some(&in1), Some(&in2)) => {
    //                     let gate_out = g.op.eval(in1, in2);
    //                     self.wires.insert(g.out.clone(), gate_out);
    //                     gates_to_remove.push(g);
    //                 }
    //                 _ => {
    //                     continue;
    //                 }
    //             }
    //         }
    //         for g in gates_to_remove {
    //             gates_to_eval.remove(g);
    //         }
    //     }
    // }

    fn get_var(&self, var: char) -> usize {
        // all the wires starting with the var prefix
        let mut wires = self
            .wires
            .iter()
            .filter(|(k, _)| k.chars().nth(0).unwrap() == var)
            .map(|k| k.clone())
            .collect::<Vec<_>>();
        // sort them (z4, z3, z2, z1, z0)
        wires.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
        // convert to a usize
        wires
            .into_iter()
            .map(|(_, &v)| v)
            .fold(0usize, |acc, b| (acc << 1) | (b as usize))
    }

    fn set_var(&mut self, var: char, value: usize) {
        // for part2, x and y are 45bits, z is 46bits
        let bits_padded = if var == 'z' {
            format!("{:046b}", value)
        } else {
            format!("{:045b}", value)
        };
        for (i, b) in bits_padded.chars().rev().enumerate() {
            let wire_name = format!("{}{:02}", var, i);
            // println!("[DDA] day24::wire_name {} to {}", wire_name,kb);
            self.wires.insert(wire_name, b == '1');
        }
    }

    pub fn debug_wires(&self) {
        let x = self.get_var('x');
        let y = self.get_var('y');
        let z = self.get_var('z');
        println!("x  {:045b}", x);
        println!("y  {:045b}", y);
        println!("z {:046b}", z);
    }

    pub fn to_mermaid_subgraph_var(&self, var: char) {
        println!("  subgraph Inputs{}", var.to_uppercase());
        println!(
            "    {}",
            (0..45)
                .map(|i| format!("{}{:02}", var, i))
                .collect::<Vec<String>>()
                .join(" & ")
        );
        println!("end");
    }

    pub fn to_mermaid(&self) {
        println!("graph TD;");
        self.to_mermaid_subgraph_var('x');
        self.to_mermaid_subgraph_var('y');

        for g in self.gates.iter() {
            println!("  {} & {} --> {}_{}[{}] --> {}", g.in1, g.in2, g.op, g.out, g.op, g.out);
        }

        self.to_mermaid_subgraph_var('z');
    }
}

fn p1(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);

    puzzle.eval();
    puzzle.get_var('z')
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);
    println!("puzzle: #gates {}, #wires {}", puzzle.gates.len(), puzzle.wires.len());

    puzzle.set_var('x', 0b100000000000000000000000000000000000000000000);
    puzzle.set_var('y', 0b100000000000000000000000000000000000000000000);

    puzzle.eval();
    puzzle.debug_wires();
    // usize::from_str_radix(&z, 2).unwrap()
    10
}

fn mermaid(input: &str) {
    let puzzle = Puzzle::from_str(input);

    // manual solution: we convert the graph to mermaid format, feed it to https://mermaid.live
    // and visually try to detect the anomalies...
    puzzle.to_mermaid();
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day24: Crossed Wires");
    // time_it(p1, "p1", "data/24_sample.txt");
    // time_it(p1, "p1", "data/24_sample2.txt");
    // time_it(p1, "p1", "data/24_input.txt");
    time_it(p2, "p2", "data/24_input.txt");

    // let input = fs::read_to_string("data/24_input2.txt").expect("cannot read sample file");
    // mermaid(&input);
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
