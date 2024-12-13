use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Machine {
    btn_a: V2,
    btn_b: V2,
    prize: V2,
}

fn parse_dirs(line: &str) -> V2 {
    let (_, dirs) = line.split_once(": ").unwrap();
    let (x_dir, y_dir) = dirs.split_once(", ").unwrap();
    V2::new(toi32(&x_dir[2..]), toi32(&y_dir[2..]))
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

fn p1(input: &str) -> usize {
    let machines = parse_machines(input);
    println!("[DDA] day13::machines {:?}", machines);
    10
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day13: Claw Contraption");
    // time_it(p1, "p1", "data/13_input.txt");
    time_it(p1, "p1", "data/13_sample.txt");
    // time_it(p2, "p2", "data/13_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/13_sample.txt"), 140);
        // assert_eq!(run_it(p2, "data/13_sample2b.txt"), 368);
    }
}
