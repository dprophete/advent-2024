use crate::utils::*;
use regex::Regex;
use std::fs;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)").unwrap();

    let sum = re
        .captures_iter(&file_content)
        .map(|caps| {
            let n1 = tou32(&caps[1]);
            let n2 = tou32(&caps[2]);
            n1 * n2
        })
        .sum::<u32>();

    println!("p1 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    re.captures_iter(&file_content).for_each(|caps| {
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

    println!("p2 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    p1("data/03_sample1.txt");
    p1("data/03_input.txt");
    p2("data/03_sample2.txt");
    p2("data/03_input.txt");
}
