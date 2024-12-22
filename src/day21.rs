use std::collections::HashMap;

use crate::utils::*;
use once_cell::sync::Lazy;

// num keypad
static NUM_KP: Lazy<Matrix<char>> = Lazy::new(|| {
    Matrix::from_vec(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ])
});

// dir keypad
static DIR_KP: Lazy<Matrix<char>> =
    Lazy::new(|| Matrix::from_vec(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]));

// num shortest paths (from, to) -> list of shortest paths
static NUM_SP: Lazy<HashMap<(char, char), Vec<Vec<char>>>> =
    Lazy::new(|| compute_matrix_shortest_paths(&NUM_KP));

// dir shortest paths (from, to) -> list of shortest paths
static DIR_SP: Lazy<HashMap<(char, char), Vec<Vec<char>>>> =
    Lazy::new(|| compute_matrix_shortest_paths(&DIR_KP));

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    codes: Vec<Vec<char>>,
}

impl Dir {
    fn to_dir_keypad(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        }
    }
}

fn pp_instr(instr: &Vec<char>) {
    for i in instr {
        print!("{}", i);
    }
    println!();
}

fn path_dir_to_path_char(path: &Vec<Dir>) -> Vec<char> {
    path.iter().map(Dir::to_dir_keypad).collect()
}

fn compute_matrix_shortest_paths(matrix: &Matrix<char>) -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut cost_at_pos = HashMap::new();
    for x0 in 0..matrix.width {
        for y0 in 0..matrix.height {
            let p0 = V2::new(x0, y0);
            if matrix.get(&p0) == Some(' ') {
                continue;
            }
            for x1 in 0..matrix.width {
                for y1 in 0..matrix.height {
                    let p1 = V2::new(x1, y1);
                    if matrix.get(&p1) == Some(' ') {
                        continue;
                    }
                    // let's compute going from p0 to p1
                    let mut paths = vec![];
                    if p0 != p1 {
                        let mut to_eplore = vec![(p0, vec![])];
                        // we already know the cost
                        let cost = (x0 - x1).abs() + (y0 - y1).abs();
                        while let Some((pos, path)) = to_eplore.pop() {
                            if path.len() > cost as usize {
                                continue;
                            }
                            if pos == p1 {
                                let mut path_chars = path_dir_to_path_char(&path);
                                path_chars.push('A');
                                paths.push(path_chars);
                                continue;
                            }
                            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                                let nx = pos.add_dir(&dir);
                                if !matrix.is_in(&nx) || matrix.get(&pos) == Some(' ') {
                                    continue;
                                }
                                let mut path_ = path.clone();
                                path_.push(dir);
                                to_eplore.push((nx, path_));
                            }
                        }
                    }
                    let c0 = matrix.get(&p0).unwrap();
                    let c1 = matrix.get(&p1).unwrap();
                    // println!("[DDA] from p0: {:?} to p1 {:?}, paths: {:?}", c0, c1, paths);
                    cost_at_pos.insert((c0, c1), paths);
                }
            }
        }
    }
    cost_at_pos
}

pub fn combine_paths(current_paths: &Vec<Vec<char>>, new_paths: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if current_paths.is_empty() {
        new_paths.clone()
    } else {
        let mut res = vec![];
        for p in current_paths {
            for path_to_c in new_paths {
                let mut p = p.clone();
                p.extend(path_to_c.clone());
                res.push(p);
            }
        }
        res
    }
}

pub fn get_paths_for_nums(code: &Vec<char>) -> Vec<Vec<char>> {
    println!("[DDA] day21:: trying to type code: {:?}", code);
    let mut arm_on = 'A';
    let mut paths_for_code: Vec<Vec<char>> = vec![];
    for &c in code {
        // need to go from arm_on to c
        let paths_to_c = NUM_SP.get(&(arm_on, c)).unwrap();
        println!(
            "from {} to {} - found {} paths",
            arm_on,
            c,
            paths_to_c.len()
        );
        paths_for_code = combine_paths(&paths_for_code, paths_to_c);
        arm_on = c;
    }
    paths_for_code
}

pub fn get_paths_for_keys(keys: &Vec<char>) -> Vec<Vec<char>> {
    println!("[DDA] day21:: trying to type keys: {:?}", keys);
    let mut arm_on = 'A';
    let mut paths_for_keys: Vec<Vec<char>> = vec![];
    for &k in keys {
        // need to go from arm_on to c
        let paths_to_k = DIR_SP.get(&(arm_on, k)).unwrap();
        println!(
            "from {} to {} - found {} paths",
            arm_on,
            k,
            paths_to_k.len()
        );
        paths_for_keys = combine_paths(&paths_for_keys, paths_to_k);
        arm_on = k;
    }
    paths_for_keys
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let codes = input.lines().map(|line| line.chars().collect()).collect();
        Puzzle { codes }
    }

    // A -> 0 : p1, p2, p3
    // [p1, p2, p3]
    // 0 -> 2 : p4, p5
    // I want to see:
    // [p1+p4, p1+p5, p2+p4, p2+p5, p3+p4, p3+p5]
    // 2-9: p5, p6
    // [p1+p4, p1+p5, p2+p4, p2+p5, p3+p4, p3+p5]
    //
    pub fn solve_p1(&self) -> usize {
        for code in &self.codes {
            let paths_for_code = get_paths_for_nums(code);
            for p in &paths_for_code {
                pp_instr(&p);
            }
            break;
        }
        10
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve_p1()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str, threshold: i32) -> usize {
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day21: Keypad Conundrum");
    time_it(p1, "p1", "data/21_sample.txt");
    // time_it(p1, "p1", "data/21_input.txt");
    // time_it(p2, "p2", "data/21_sample.txt");
    // time_it(p2, "p2", "data/21_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/21_sample.txt"), 126384);
        // assert_eq!(run_it(p1, "data/21_input.txt"), 242);
        // assert_eq!(run_it(p2, "data/21_sample.txt"), 16);
        // assert_eq!(run_it(p2, "data/21_input.txt"), 595975512785325);
    }
}
