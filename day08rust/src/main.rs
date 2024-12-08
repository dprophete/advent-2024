#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::{fs, time::Instant};

use itertools::Itertools;
use utils::{fmt_t, v2_add, v2_sub, Matrix, V2};

// find antennas: we build a hahsmap: antenna (char) -> list of positions (vec<V2>)
fn get_antennas(matrix: &Matrix) -> HashMap<char, Vec<V2>> {
    let mut antennas: HashMap<char, Vec<V2>> = HashMap::new();
    for y in 0..matrix.size {
        for x in 0..matrix.size {
            let pos = (x, y);
            match matrix.get(pos) {
                None | Some('.') => {}
                Some(a) => {
                    antennas
                        .entry(a)
                        .and_modify(|v| v.push(pos))
                        .or_insert(vec![pos]);
                }
            }
        }
    }
    antennas
}

fn get_pairs(positions: &Vec<V2>) -> Vec<(V2, V2)> {
    positions
        .iter()
        .combinations(2)
        .map(|v| (*v[0], *v[1]))
        .collect()
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn p1(input: &str) {
    let start_t = Instant::now();
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let matrix = Matrix::from_str(&file_content);
    let antennas = get_antennas(&matrix);

    let mut antinodes: HashSet<V2> = HashSet::new();
    for (_antenna, positions) in antennas.iter() {
        let pairs: Vec<(V2, V2)> = get_pairs(&positions);

        for (a1, a2) in pairs {
            let diff = v2_sub(a2, a1);
            let an1 = v2_sub(a1, diff);
            let an2 = v2_add(a2, diff);
            if matrix.is_in(an1) {
                antinodes.insert(an1);
            }
            if matrix.is_in(an2) {
                antinodes.insert(an2);
            }
        }
    }

    let sum = antinodes.len();
    println!("[{}] p1 {} -> {}", fmt_t(start_t), input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) {
    let start_t = Instant::now();
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let matrix = Matrix::from_str(&file_content);
    let antennas = get_antennas(&matrix);

    let mut antinodes: HashSet<V2> = HashSet::new();
    for (_antenna, positions) in antennas.iter() {
        let pairs: Vec<(V2, V2)> = get_pairs(&positions);

        for (a1, a2) in pairs {
            let diff = v2_sub(a2, a1);

            let mut p = a1.clone();
            while matrix.is_in(p) {
                antinodes.insert(p);
                p = v2_sub(p, diff);
            }
            p = a2.clone();
            while matrix.is_in(p) {
                antinodes.insert(p);
                p = v2_add(p, diff);
            }
        }
    }

    let sum = antinodes.len();
    println!("[{}] p2 {} -> {}", fmt_t(start_t), input, sum);
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
