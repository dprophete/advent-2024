use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Machine {
    btn_a: (i64, i64),
    btn_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    pub fn from_str(s: &str) -> Machine {
        let mut lines = s.lines();
        let btn_a = parse_dirs(lines.next().unwrap());
        let btn_b = parse_dirs(lines.next().unwrap());
        let prize = parse_dirs(lines.next().unwrap());
        Machine { btn_a, btn_b, prize }
    }

    pub fn compute_cost(&self, offset: i64, enforce_limit: bool) -> i64 {
        let (ax, ay) = self.btn_a;
        let (bx, by) = self.btn_b;
        let (px, py) = self.prize;

        let (px, py) = (px + offset, py + offset);
        let det = ax * by - ay * bx;
        let num_a = by * px - bx * py;
        let num_b = ax * py - ay * px;
        if num_a % det != 0 || num_b % det != 0 {
            return 0;
        }
        let nb_a = num_a / det;
        let nb_b = num_b / det;
        if enforce_limit && (nb_a > 100 || nb_b > 100) {
            return 0;
        }
        nb_a * 3 + nb_b
    }
}

// parse directions: "Button A: X+26, Y+66" -> (26, 66)
fn parse_dirs(line: &str) -> (i64, i64) {
    let (_, dirs) = line.split_once(": ").unwrap();
    let (x_dir, y_dir) = dirs.split_once(", ").unwrap();
    (toi64(&x_dir[2..]), toi64(&y_dir[2..]))
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input.split("\n\n").map(Machine::from_str).collect()
}

fn p1(input: &str) -> i64 {
    let mut sum = 0;
    for machine in parse_machines(input) {
        sum += machine.compute_cost(0, true);
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> i64 {
    let mut sum = 0;
    for machine in parse_machines(input) {
        sum += machine.compute_cost(10000000000000_i64, false);
    }
    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day13: Claw Contraption");
    time_it(p1, "p1", "data/13_input.txt");
    time_it(p2, "p2", "data/13_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/13_sample.txt"), 480);
        assert_eq!(run_it(p2, "data/13_sample.txt"), 875318608908);
    }
}
