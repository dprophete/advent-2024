#![allow(dead_code)]

use std::{fs, time::Instant};

fn toi64(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

fn format_d(duration: std::time::Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_micros() > 0 {
        format!("{}µs", duration.as_micros())
    } else {
        format!("{}ns", duration.as_nanos())
    }
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

fn p1(input: &str) {
    let start_t = Instant::now();
    let equations = parse_file(input);

    let mut sum = 0;
    for (total, lst) in equations {
        if p1_is_equation_valid(total, &lst) {
            sum += total;
        }
    }
    println!("[{}] p1 {} -> {}", format_d(start_t.elapsed()), input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2_is_equation_valid(total: i64, lst: &[i64]) -> bool {
    // we are keeping the string representation of the equation to be able to print it for
    // debugging....
    let mut nbs = vec![(lst[0], format!("{}", lst[0]))];

    for nx in lst.iter().skip(1) {
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
    let start_t = Instant::now();
    let equations = parse_file(input);

    let mut sum = 0;
    for (total, lst) in equations {
        if p2_is_equation_valid(total, &lst) {
            sum += total;
        }
    }
    println!("[{}] p2 {} -> {}", format_d(start_t.elapsed()), input, sum);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    // p1("sample.txt");
    p1("input.txt");
    p2("sample.txt");
    // p2("input.txt");
}
