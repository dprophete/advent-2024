/// sort lists
use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

// fn parse_line(line: &str) -> (i32, i32) {
//     let vals = line.split_ascii_whitespace().map(toi32).collect::<Vec<_>>();
//     let &[l, r] = vals.array_chunks::<2>().next().unwrap();
//     (l, r)
// }

fn p1(input: &str) -> i32 {
    // let (left, right): (Vec<i32>, Vec<i32>) = input.lines().map(parse_line).unzip();
    let sum = 0;
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> i32 {
//     // let (left, right): (Vec<i32>, Vec<i32>) = input.lines().map(parse_line).unzip();
//     let sum = 0;
//     sum
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day12: sort lists");
    time_it(p1, "p1", "data/12_samplea.txt");
    time_it(p1, "p1", "data/12_sampleb.txt");
    time_it(p1, "p1", "data/12_samplec.txt");
    // time_it(p1, "p1", "data/12_input.txt");
    // time_it(p2, "p2", "data/12_sample.txt");
    // time_it(p2, "p2", "data/12_input.txt"); // takes a few seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/12_samplea.txt"), 140);
        assert_eq!(run_it(p1, "data/12_sampleb.txt"), 772);
        assert_eq!(run_it(p1, "data/12_samplec.txt"), 1930);
    }
}
