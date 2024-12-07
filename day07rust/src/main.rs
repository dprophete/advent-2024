#![allow(dead_code)]

use std::fs;

use itertools::Itertools;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn is_equation_valid(total: i64, lst: Vec<i64>) -> bool {
    let mut nbs = vec![lst[0]];

    for i in 1..lst.len() {
        let nx = lst[i];
        nbs = nbs
            .iter()
            .flat_map(|acc| vec![acc + nx, acc * nx])
            .take_while_inclusive(|&x| x != total)
            .collect();
        if *nbs.last().unwrap() == total {
            return true;
        }
    }
    false
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let equations = file_content
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            (
                lhs.parse::<i64>().unwrap(),
                rhs.split(" ")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect::<Vec<(i64, Vec<i64>)>>();

    let mut sum = 0;
    for (total, lst) in equations {
        if is_equation_valid(total, lst) {
            sum += total;
        }
    }
    println!("p1 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    // p2("sample.txt");
    // p2("input.txt");
}
