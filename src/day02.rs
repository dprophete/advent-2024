use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace().map(toi32).collect::<Vec<_>>()
}

fn is_safe_p1(line: &[i32]) -> bool {
    let &[v1, v2] = line.array_chunks::<2>().next().unwrap();
    let is_going_up = (v2 - v1) > 0;

    for window in line.windows(2) {
        let [a, b] = &window else { unreachable!() };
        let current_diff = b - a;
        if current_diff.abs() < 1 || current_diff.abs() > 3 {
            return false;
        }
        let current_going_up = b - a > 0;
        if current_going_up != is_going_up {
            return false;
        }
    }
    true
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|l| is_safe_p1(l))
        .count()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn line_without_idx(line: &[i32], i: usize) -> Vec<i32> {
    line.iter()
        .enumerate()
        .filter_map(|(idx, &v)| if idx != i { Some(v) } else { None })
        .collect()
}

fn is_safe_p2(line: &[i32]) -> bool {
    (0..line.len())
        .map(|i| line_without_idx(line, i))
        .any(|modified_line| is_safe_p1(&modified_line))
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|l| is_safe_p2(l))
        .count()
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    time_it(p1, "data/02_sample.txt");
    time_it(p1, "data/02_input.txt");
    time_it(p2, "data/02_sample.txt");
    time_it(p2, "data/02_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/02_sample.txt"), 2);
        assert_eq!(run_it(p2, "data/02_sample.txt"), 4);
    }
}
