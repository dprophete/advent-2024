use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Machine {
    btn_a: (i32, i32),
    btn_b: (i32, i32),
    prize: (i32, i32),
}

fn parse_dirs(line: &str) -> (i32, i32) {
    let (_, dirs) = line.split_once(": ").unwrap();
    let (x_dir, y_dir) = dirs.split_once(", ").unwrap();
    (toi32(&x_dir[2..]), toi32(&y_dir[2..]))
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let line_a = lines.next().unwrap();
            let line_b = lines.next().unwrap();
            let line_prize = lines.next().unwrap();
            let btn_a = parse_dirs(line_a);
            let btn_b = parse_dirs(line_b);
            let prize = parse_dirs(line_prize);
            Machine {
                btn_a,
                btn_b,
                prize,
            }
        })
        .collect()
}

fn p1(input: &str) -> i32 {
    let mut sum = 0;
    for machine in parse_machines(input) {
        let Machine {
            btn_a,
            btn_b,
            prize,
        } = machine;
        let (ax, ay) = btn_a;
        let (bx, by) = btn_b;
        let (px, py) = prize;
        let det = ax * by - ay * bx;
        let nb_a = (by * px - bx * py) as f32 / det as f32;
        let nb_b = (ax * py - ay * px) as f32 / det as f32;
        if nb_a.fract() != 0.0 || nb_b.fract() != 0.0 || nb_a > 100.0 || nb_b > 100.0 {
            continue;
        }
        sum += (nb_a as i32) * 3 + (nb_b as i32) * 1;
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day13: Claw Contraption");
    time_it(p1, "p1", "data/13_input.txt");
    // time_it(p1, "p1", "data/13_sample.txt");
    // time_it(p2, "p2", "data/13_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/13_sample.txt"), 480);
        // assert_eq!(run_it(p2, "data/13_sample2b.txt"), 368);
    }
}
