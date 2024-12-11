/// RPN calculator
use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            (toi64(lhs), rhs.split(" ").map(toi64).collect::<Vec<i64>>())
        })
        .collect::<Vec<(i64, Vec<i64>)>>()
}

fn p1_is_equation_valid(total: i64, lst: &[i64]) -> bool {
    let mut nbs = vec![lst[0]];

    for nx in lst.iter().skip(1) {
        nbs = nbs
            .iter()
            .flat_map(|acc| vec![acc + nx, acc * nx])
            .collect();
    }
    nbs.contains(&total)
}

fn p1(input: &str) -> i64 {
    let equations = parse_input(input);

    let mut sum = 0;
    for (total, lst) in equations {
        if p1_is_equation_valid(total, &lst) {
            sum += total;
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2_is_equation_valid(total: i64, lst: &[i64]) -> bool {
    let mut nbs = vec![lst[0]];

    for nx in lst.iter().skip(1) {
        nbs = nbs
            .iter()
            .flat_map(|acc| vec![acc + nx, acc * nx, toi64(&format!("{}{}", acc, nx))])
            .collect();
    }
    for &acc in nbs.iter() {
        if acc == total {
            return true;
        }
    }
    false
}

fn p2(input: &str) -> i64 {
    let equations = parse_input(input);

    let mut sum = 0;
    for (total, lst) in equations {
        if p2_is_equation_valid(total, &lst) {
            sum += total;
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// run
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day07: RPN calculator");
    time_it(p1, "p1", "data/07_sample.txt");
    time_it(p1, "p1", "data/07_input.txt");
    time_it(p2, "p2", "data/07_sample.txt");
    time_it(p2, "p2", "data/07_input.txt"); // takes 3s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/07_sample.txt"), 3749);
        assert_eq!(run_it(p2, "data/07_sample.txt"), 11387);
    }
}
