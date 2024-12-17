use std::convert::identity;

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
        Machine { a, b, c, prg }
    }
}

fn p1(input: &str) -> i32 {
    let mut machine = Machine::from_str(input);
    println!("[DDA] day17::machine {:?}", machine);
    let mut sum = 0;
    sum
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
    // time_it(p1, "p1", "data/17_input.txt");
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
