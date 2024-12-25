use std::convert::identity;

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    keys: Vec<Vec<i32>>,
    locks: Vec<Vec<i32>>,
}

fn key_fitst_lock(key: &Vec<i32>, lock: &Vec<i32>) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] >= 6 {
            return false;
        }
    }
    true
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let mut keys = vec![];
        let mut locks = vec![];
        let groups = input.split("\n\n").collect::<Vec<_>>();
        for &g in &groups {
            let m = Matrix::from_str(g, identity);
            // keys have the top row filled with '#'
            let is_lock = (0..m.width).all(|x| m.get(&V2::new(x, 0)) == Some('#'));
            if is_lock {
                let mut heights = vec![];
                for x in 0..m.width {
                    let mut height = m.height - 1;
                    for y in 1..m.height {
                        if m.get(&V2::new(x, y)) == Some('.') {
                            height = y - 1;
                            break;
                        }
                    }
                    heights.push(height);
                }
                locks.push(heights);
            } else {
                let mut heights = vec![];
                for x in 0..m.width {
                    let mut height = 0;
                    for y in 0..m.height - 1 {
                        if m.get(&V2::new(x, y)) == Some('#') {
                            height = m.height - y - 1;
                            break;
                        }
                    }
                    heights.push(height);
                }
                keys.push(heights);
            }
        }
        Puzzle { keys, locks }
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);

    let mut nb_fits = 0;
    for key in &puzzle.keys {
        for lock in &puzzle.locks {
            if key_fitst_lock(key, lock) {
                nb_fits += 1;
            }
        }
    }
    nb_fits
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> String {
//     let puzzle = Puzzle::from_str(input);
//     puzzle.p2()
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day25: Code Chronicle");
    time_it(p1, "p1", "data/25_sample.txt");
    time_it(p1, "p1", "data/25_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/25_sample.txt"), 3);
        assert_eq!(run_it(p1, "data/25_input.txt"), 3155);
    }
}
