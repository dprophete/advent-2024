#![feature(array_chunks)]
#![allow(dead_code)]
use std::fs;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn is_safe(line: &Vec<i32>) -> bool {
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

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let nb_safe = file_content.lines().map(parse_line).filter(is_safe).count();

    println!("p1 sum for {} -> {}", input, nb_safe);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) {
//     let file_content = fs::read_to_string(input).expect("cannot read sample file");
//
//     let (left, right): (Vec<i32>, Vec<i32>) = file_content.lines().map(parse_line).unzip();
//     let mut sum = 0;
//     for l in left.iter() {
//         // not super optimized, but it works
//         let nb_matches: i32 = right.iter().filter(|r| *r == l).count() as i32;
//         sum += nb_matches * l;
//     }
//
//     println!("p2 sum for {} -> {}", input, sum);
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    // p2("sample.txt");
    // p2("input.txt");
}
