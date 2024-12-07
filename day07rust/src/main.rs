#![allow(dead_code)]

use std::fs;

fn toi64(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_file(input: &str) -> Vec<(i64, Vec<i64>)> {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    file_content
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            (toi64(lhs), rhs.split(" ").map(toi64).collect::<Vec<i64>>())
        })
        .collect::<Vec<(i64, Vec<i64>)>>()
}

fn p1_is_equation_valid(total: i64, lst: Vec<i64>) -> bool {
    let mut nbs = vec![lst[0]];

    for i in 1..lst.len() {
        let nx = lst[i];
        nbs = nbs
            .iter()
            .flat_map(|acc| vec![acc + nx, acc * nx])
            .collect();
    }
    nbs.contains(&total)
}

fn p1(input: &str) {
    let equations = parse_file(input);

    let mut sum = 0;
    for (total, lst) in equations {
        if p1_is_equation_valid(total, lst) {
            sum += total;
        }
    }
    println!("p1 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2_is_equation_valid(total: i64, lst: &Vec<i64>) -> bool {
    let mut nbs = vec![(lst[0], format!("{}", lst[0]))];

    for i in 1..lst.len() {
        let nx = lst[i];
        nbs = nbs
            .iter()
            .flat_map(|(acc, str)| {
                vec![
                    (acc + nx, format!("{} + {}", str, nx)),
                    (acc * nx, format!("{} * {}", str, nx)),
                    (
                        toi64(&format!("{}{}", acc, nx)),
                        format!("{} || {}", str, nx),
                    ),
                ]
            })
            .collect();
    }
    for (acc, str) in nbs.iter() {
        if *acc == total {
            println!("{} = {}", acc, str);
            return true;
        }
    }
    false
}

fn p2(input: &str) {
    let equations = parse_file(input);

    let mut sum = 0;
    for (total, lst) in equations {
        if p2_is_equation_valid(total, &lst) {
            sum += total;
        }
    }
    println!("p2 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    p2("sample.txt");
    // p2("input.txt");
}
