use core::fmt;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::utils::*;

//--------------------------------------------------------------------------------
// op
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
        write!(f, "{}", s)
    }
}

//--------------------------------------------------------------------------------
// gate
//--------------------------------------------------------------------------------

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

    pub fn with_rename(&self, rename_map: &HashMap<String, String>) -> Gate {
        Gate {
            in1: rename_map.get(&self.in1).unwrap_or(&self.in1).clone(),
            in2: rename_map.get(&self.in2).unwrap_or(&self.in2).clone(),
            out: rename_map.get(&self.out).unwrap_or(&self.out).clone(),
            op: self.op.clone(),
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // sort the inputs.. it makes it easier to read the output
        if self.in1.cmp(&self.in2) == std::cmp::Ordering::Less {
            write!(f, "{} {} {} -> {}", self.in1, self.op, self.in2, self.out)
        } else {
            write!(f, "{} {} {} -> {}", self.in2, self.op, self.in1, self.out)
        }
    }
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

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

    // reset all computed wires (only keep x and y)
    pub fn reset_wires(&mut self) {
        let keys = self.wires.keys().cloned().collect::<Vec<_>>();
        for wire in keys {
            match wire.chars().nth(0).unwrap() {
                'x' | 'y' => {}
                _ => {
                    self.wires.remove(&wire);
                }
            }
        }
    }

    // run machine
    pub fn eval(&mut self) {
        self.reset_wires();

        // now ready to eval
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
            let mut wires_to_insert = HashMap::new();
            for &g in gates_to_eval.iter() {
                match (self.wires.get(&g.in1), self.wires.get(&g.in2)) {
                    (Some(&in1), Some(&in2)) => {
                        let gate_out = g.op.eval(in1, in2);
                        // self.wires.insert(g.out.clone(), gate_out);
                        wires_to_insert.insert(g.out.clone(), gate_out);
                        gates_to_remove.push(g);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            self.wires.extend(wires_to_insert);

            for g in gates_to_remove {
                gates_to_eval.remove(g);
            }
        }
    }

    // get the value of a var (x, y, z) by reading all the x01, x02, etc... wires
    fn get_var(&self, var: char) -> usize {
        // all the wires starting with the var prefix
        let mut wires = self
            .wires
            .iter()
            .filter(|(k, _)| k.starts_with(var))
            .collect::<Vec<_>>();
        // sort them (z4, z3, z2, z1, z0)
        wires.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
        // convert to a usize
        wires
            .into_iter()
            .map(|(_, &v)| v)
            .fold(0usize, |acc, b| (acc << 1) | (b as usize))
    }

    // set the value of a var (x, y, z) by setting all the x01, x02, etc... wires
    fn set_var(&mut self, var: char, value: usize) {
        // for part2, x and y are 45bits, z is 46bits
        let bits_padded = if var == 'z' {
            format!("{:046b}", value)
        } else {
            format!("{:045b}", value)
        };
        for (i, b) in bits_padded.chars().rev().enumerate() {
            let wire_name = format!("{}{:02}", var, i);
            self.wires.insert(wire_name, b == '1');
        }
    }

    // we are going to rename the wires to make it easier to debug
    // we want to get to a point where for each bit, we have the following:
    //
    //   C03 AND XOR_x03_y03 -> TMP03
    //   XOR_x03_y03 XOR C03 -> z03
    //   TMP03 OR AND_y03_x03 -> C04
    //
    // let's compute a map of old-name -> new-name
    pub fn compute_rename_map(&self) -> HashMap<String, String> {
        let mut rename_map = HashMap::new();
        // let first rename all the x01 AND y01, x01 XOR y01 -> XOR_x01_y01, and AND_x01_y01
        for g in self.gates.clone().iter() {
            if g.in1.starts_with('x') && g.in2.starts_with('y') {
                rename_map.insert(g.out.clone(), format!("{}_{}_{}", g.op, g.in1, g.in2));
            }
            if g.in1.starts_with('y') && g.in2.starts_with('x') {
                rename_map.insert(g.out.clone(), format!("{}_{}_{}", g.op, g.in2, g.in1));
            }
        }
        // find the carries -> C02
        // we detect XOR_z03_C03 XOR C03 -> z03
        for g in self.gates.clone().iter() {
            if g.out.starts_with('z') && g.op == Op::Xor {
                if rename_map.contains_key(&g.in1) {
                    // carry is 2nd one
                    rename_map.insert(g.in2.clone(), format!("C{}", &g.out.clone()[1..].to_string()));
                } else if rename_map.contains_key(&g.in2) {
                    rename_map.insert(g.in1.clone(), format!("C{}", &g.out.clone()[1..].to_string()));
                }
            }
        }
        // find the tmp ones -> TMP02
        // we detect C03 AND XOR_x03_y03 -> TMP03
        for g in self.gates.clone().iter() {
            let g = g.with_rename(&rename_map);
            if g.in1.starts_with('C') && g.op == Op::And {
                rename_map.insert(g.out.clone(), format!("TMP{}", &g.in1.clone()[1..].to_string()));
            }
            if g.in2.starts_with('C') && g.op == Op::And {
                rename_map.insert(g.out.clone(), format!("TMP{}", &g.in2.clone()[1..].to_string()));
            }
        }
        rename_map
    }

    pub fn eval_with_rename(&mut self) {
        self.reset_wires();

        let rename_map = self.compute_rename_map();

        // now ready to eval
        let mut gates_to_eval: HashSet<&Gate> = HashSet::new();
        for g in self.gates.iter() {
            gates_to_eval.insert(g);
        }

        // we are going to evaluate all the gates which have their wire ready
        // and then go back to the beginning until all gates are evaluated
        // note: this is not optimal at all...
        // ideally we build a proper graph of gate evaluation order
        let mut nb_iter = 0;
        while !gates_to_eval.is_empty() {
            let mut gates_to_remove = vec![];
            let mut wires_to_insert = HashMap::new();
            for &g in gates_to_eval.iter() {
                match (self.wires.get(&g.in1), self.wires.get(&g.in2)) {
                    (Some(&in1), Some(&in2)) => {
                        let gate_out = g.op.eval(in1, in2);
                        // self.wires.insert(g.out.clone(), gate_out);
                        wires_to_insert.insert(g.out.clone(), gate_out);
                        gates_to_remove.push(g);
                    }
                    _ => {
                        continue;
                    }
                }
            }

            // display the section for each bit
            // for each bit, we want to see:
            //   C03 AND XOR_x03_y03 -> TMP03
            //   XOR_x03_y03 XOR C03 -> z03
            //   TMP03 OR AND_y03_x03 -> C04
            //
            // if it's not exactly that, there is an error with the wires
            let current_bit = (nb_iter + 1) / 2;

            if current_bit > 2 {
                if nb_iter % 2 == 1 {
                    println!("-- bit {}", current_bit);
                }
                for g in gates_to_remove.iter() {
                    println!("{} (was {})", g.with_rename(&rename_map), g);
                }
            }

            self.wires.extend(wires_to_insert);
            for g in gates_to_remove {
                gates_to_eval.remove(g);
            }

            nb_iter += 1;
        }
    }

    pub fn check_add(&mut self, x: usize, y: usize) -> bool {
        self.set_var('x', x);
        self.set_var('y', y);
        self.eval();
        let z = self.get_var('z');
        if x + y != z {
            println!("bad:");
            println!("  x  {:045b}", x);
            println!("  y  {:045b}", y);
            println!("  z {:046b}", z);
            return false;
        }
        true
    }

    pub fn swap_wires(&mut self, wire1: &str, wire2: &str) {
        self.rename_wire(wire1, "TMP_SWAP");
        self.rename_wire(wire2, wire1);
        self.rename_wire("TMP_SWAP", wire2);
    }

    pub fn rename_wire(&mut self, old: &str, new: &str) {
        // first rename wire it exists
        if let Some(value) = self.wires.remove(old) {
            self.wires.insert(String::from(new), value);
        }
        // then rename wire in gates
        for g in self.gates.iter_mut() {
            if g.out == old {
                g.out = String::from(new);
            }
        }
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

fn p2(input: &str) -> String {
    let mut puzzle = Puzzle::from_str(input);

    // without swapping any wires we find issues with bits 11, 12, 25, 26, 31, 36
    //
    // bit 11 & bit 12
    puzzle.swap_wires("kth", "z12");
    // bit 25 & bit 26
    puzzle.swap_wires("gsd", "z26");
    // bit 31
    puzzle.swap_wires("tbt", "z32");
    // bit 36
    puzzle.swap_wires("vpm", "qnf");

    // after swapping we shouldn't see any errors
    let mut found_error = false;
    for i in 0..45 {
        let x = 1 << i;
        let y = 1 << i;
        if !puzzle.check_add(x, y) {
            println!("  ^^ for bit {}", i);
            found_error = true;
        }
    }

    if found_error {
        // you need to inspect visually the debug output
        puzzle.eval_with_rename();
        String::from("still has errors")
    } else {
        String::from("gsd,kth,qnf,tbt,vpm,z12,z26,z32")
    }
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day24: Crossed Wires");
    time_it(p1, "p1", "data/24_sample.txt");
    time_it(p1, "p1", "data/24_sample2.txt");
    time_it(p1, "p1", "data/24_input.txt");
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
        assert_eq!(run_it(p2, "data/24_input.txt"), "gsd,kth,qnf,tbt,vpm,z12,z26,z32");
    }
}
