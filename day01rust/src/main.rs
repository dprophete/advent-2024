#![allow(dead_code)]

use std::fs;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn str_to_list_of_ints(s: &str) -> Vec<i64> {
    s.split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> (i32, i32) {
    let vals = line
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [l, r, ..] = vals.as_slice() else {
        panic!("invalid input")
    };
    (*l, *r)
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = file_content.lines().map(parse_line).unzip();
    left.sort();
    right.sort();

    let pairs: Vec<(i32, i32)> = left.into_iter().zip(right).collect();
    let sum = pairs.iter().map(|(l, r)| (r - l).abs()).sum::<i32>();

    println!("p1 sum: {}", sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn first_win(mut time_push: i64, t: i64, d: i64) -> i64 {
//     while time_push < t - 1 {
//         let time_remaining = t - time_push;
//         let final_distance = time_remaining * time_push;
//         if final_distance > d {
//             return time_push;
//         }
//         time_push += 1
//     }
//     return time_push;
// }
//
// fn last_win(mut time_push: i64, t: i64, d: i64) -> i64 {
//     while time_push > 0 {
//         let time_remaining = t - time_push;
//         let final_distance = time_remaining * time_push;
//         if final_distance > d {
//             return time_push;
//         }
//         time_push -= 1
//     }
//     return time_push;
// }

// fn p2(input: &str) {
//     // let mut sum = 0;
//     let mut file_content = fs::read_to_string(input).expect("cannot read sample file");
//     file_content.pop();
//
//     let lines = file_content
//         .split("\n")
//         .map(|line| {
//             let (_, nbs_str) = line.split_once(":").unwrap();
//             nbs_str.replace(" ", "").parse::<i64>().unwrap()
//         })
//         .collect::<Vec<_>>();
//
//     let t = *(&lines[0]);
//     let d = *(&lines[1]);
//     // we could do brute force
//     // let nb_wins = nb_wins(t, d);
//     // println!("p2 res for {}: {}", input, nb_wins);
//
//     // or be smarter
//     let first_win_ = first_win(0, t, d);
//     let last_win_ = last_win(t - 1, t, d);
//     println!("p2 res for {}: {}", input, last_win_ - first_win_ + 1);
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
