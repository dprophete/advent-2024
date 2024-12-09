use crate::utils::*;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// find antennas: we build a hahsmap: antenna (char) -> list of positions (vec<V2>)
fn get_antennas(matrix: &Matrix) -> HashMap<char, Vec<V2>> {
    let mut antennas: HashMap<char, Vec<V2>> = HashMap::new();
    for y in 0..matrix.size {
        for x in 0..matrix.size {
            let pos = V2::new(x, y);
            match matrix.get(&pos) {
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

fn get_pairs(positions: &[V2]) -> Vec<(V2, V2)> {
    // for p1 in positions.iter() {
    //     for p2 in positions.iter() {
    //         if p1 != p2 {
    //             (p1, p2);
    //         }
    //     }
    // }
    positions
        .iter()
        .combinations(2)
        .map(|v| (*v[0], *v[1]))
        .collect()
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn p1(input: &str) -> usize {
    let matrix = Matrix::from_str(input);
    let antennas = get_antennas(&matrix);

    let mut antinodes: HashSet<V2> = HashSet::new();
    for (_antenna, positions) in antennas.iter() {
        let pairs: Vec<(V2, V2)> = get_pairs(positions);

        for (a1, a2) in pairs {
            let diff = a2.sub(&a1);
            let an1 = a1.sub(&diff);
            let an2 = a2.add(&diff);
            if matrix.is_in(&an1) {
                antinodes.insert(an1);
            }
            if matrix.is_in(&an2) {
                antinodes.insert(an2);
            }
        }
    }

    antinodes.len()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let matrix = Matrix::from_str(input);
    let antennas = get_antennas(&matrix);

    let mut antinodes: HashSet<V2> = HashSet::new();
    for (_antenna, positions) in antennas.iter() {
        let pairs: Vec<(V2, V2)> = get_pairs(positions);

        for (a1, a2) in pairs {
            let diff = a2.sub(&a1);

            let mut p = a1;
            while matrix.is_in(&p) {
                antinodes.insert(p);
                p = p.sub(&diff);
            }
            p = a2;
            while matrix.is_in(&p) {
                antinodes.insert(p);
                p = p.add(&diff);
            }
        }
    }

    antinodes.len()
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    time_it(p1, "data/08_sample.txt");
    time_it(p1, "data/08_input.txt");
    time_it(p2, "data/08_sample.txt");
    time_it(p2, "data/08_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(run_it(p1, "data/08_sample.txt"), 14);
    }

    #[test]
    fn test_p2() {
        assert_eq!(run_it(p2, "data/08_sample.txt"), 34);
    }
}
