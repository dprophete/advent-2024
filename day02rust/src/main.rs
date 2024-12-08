#![feature(array_chunks)]
#![allow(dead_code)]

use std::fs;
use utils::toi32;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace().map(toi32).collect::<Vec<_>>()
}

fn is_safe_p1(line: &Vec<i32>) -> bool {
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

    let nb_safe = file_content
        .lines()
        .map(parse_line)
        .filter(is_safe_p1)
        .count();

    println!("p1 sum for {} -> {}", input, nb_safe);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn line_without_idx(line: &Vec<i32>, i: usize) -> Vec<i32> {
    line.iter()
        .enumerate()
        .filter_map(|(idx, &v)| if idx != i { Some(v) } else { None })
        .collect()
}

fn is_safe_p2(line: &Vec<i32>) -> bool {
    (0..line.len())
        .map(|i| line_without_idx(line, i))
        .any(|modified_line| is_safe_p1(&modified_line))
}

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let nb_safe = file_content
        .lines()
        .map(parse_line)
        .filter(is_safe_p2)
        .count();

    println!("p2 sum for {} -> {}", input, nb_safe);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    p2("sample.txt");
    p2("input.txt");
}
