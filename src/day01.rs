use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_line(line: &str) -> (i32, i32) {
    let vals = line.split_ascii_whitespace().map(toi32).collect::<Vec<_>>();
    let &[l, r] = vals.array_chunks::<2>().next().unwrap();
    (l, r)
}

fn p1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.lines().map(parse_line).unzip();
    left.sort();
    right.sort();

    let pairs: Vec<(i32, i32)> = left.into_iter().zip(right).collect();
    let sum = pairs.iter().map(|(l, r)| (r - l).abs()).sum::<i32>();
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> i32 {
    let (left, right): (Vec<i32>, Vec<i32>) = input.lines().map(parse_line).unzip();
    let mut sum = 0;
    for l in left.iter() {
        // not super optimized, but it works
        let nb_matches: i32 = right.iter().filter(|r| *r == l).count() as i32;
        sum += nb_matches * l;
    }

    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day01: sort lists");
    time_it(p1, "p1", "data/01_sample.txt");
    time_it(p2, "p2", "data/01_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/01_sample.txt"), 11);
        assert_eq!(run_it(p2, "data/01_sample.txt"), 31);
    }
}
