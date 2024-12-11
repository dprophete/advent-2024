/// mul do and don't
use crate::utils::*;
use regex::Regex;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn p1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)").unwrap();

    let sum = re
        .captures_iter(input)
        .map(|caps| {
            let n1 = tou32(&caps[1]);
            let n2 = tou32(&caps[2]);
            n1 * n2
        })
        .sum::<u32>();
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    re.captures_iter(input).for_each(|caps| {
        if caps[0].to_string() == "do()" {
            enabled = true
        } else if caps[0].to_string() == "don't()" {
            enabled = false
        } else if enabled {
            let n1 = tou32(&caps[1]);
            let n2 = tou32(&caps[2]);
            sum += n1 * n2
        }
    });
    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day03: mul do and don't");
    time_it(p1, "p1", "data/03_sample1.txt");
    time_it(p1, "p1", "data/03_input.txt");
    time_it(p2, "p2", "data/03_sample2.txt");
    time_it(p2, "p2", "data/03_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/03_sample1.txt"), 161);
        assert_eq!(run_it(p2, "data/03_sample2.txt"), 48);
    }
}
