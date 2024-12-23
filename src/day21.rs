use std::collections::HashMap;

use crate::utils::*;
use once_cell::sync::Lazy;

type PathD = Vec<Dir>;
type PathC = Vec<char>;

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
static NUM_SP: Lazy<HashMap<(char, char), Vec<PathC>>> =
    Lazy::new(|| compute_matrix_shortest_paths(&NUM_KP));

// dir shortest paths (from, to) -> list of shortest paths
static DIR_SP: Lazy<HashMap<(char, char), Vec<PathC>>> =
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

fn pp_instr(instr: &PathC) {
    for i in instr {
        print!("{}", i);
    }
    println!();
}

fn path_dir_to_path_char(path: &PathD) -> PathC {
    path.iter().map(Dir::to_dir_keypad).collect()
}

fn compute_matrix_shortest_paths(matrix: &Matrix<char>) -> HashMap<(char, char), Vec<PathC>> {
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
                    if p0 == p1 {
                        paths.push(vec!['A']);
                    } else {
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

pub fn combine_paths(current_paths: &Vec<PathC>, new_paths: &Vec<PathC>) -> Vec<PathC> {
    if current_paths.is_empty() {
        new_paths.clone()
    } else if new_paths.is_empty() {
        current_paths.clone()
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

pub fn get_paths_for_nums(code: &Vec<char>) -> Vec<PathC> {
    let mut pos_arm = 'A';
    let mut paths_for_code: Vec<Vec<char>> = vec![];
    for &c in code {
        // need to go from pos_arm to c
        let paths_to_c = NUM_SP.get(&(pos_arm, c)).unwrap();
        paths_for_code = combine_paths(&paths_for_code, paths_to_c);
        pos_arm = c;
    }
    paths_for_code
}

pub fn get_paths_for_keys(keys: &PathC) -> Vec<PathC> {
    let mut pos_arm = 'A';
    let mut paths_for_keys: Vec<Vec<char>> = vec![];
    for &k in keys {
        // need to go from pos_arm to c
        let paths_to_k = DIR_SP.get(&(pos_arm, k)).unwrap();
        paths_for_keys = combine_paths(&paths_for_keys, paths_to_k);
        pos_arm = k;
    }
    paths_for_keys
}

// let's iterate over the paths and count how many times we change letter
pub fn score(path: &PathC) -> usize {
    let mut score = 0;
    let mut last = ' ';
    for &c in path {
        if c != last {
            score += 1;
            last = c;
        }
    }
    score
}

pub fn only_keep_good_paths(paths: &Vec<PathC>) -> Vec<PathC> {
    let min_lens = paths.iter().map(|p| score(p)).min().unwrap();
    let paths: Vec<PathC> = paths
        .iter()
        .filter(|&p| score(p) == min_lens)
        .map(|p| p.clone())
        .collect();
    // paths.sort();
    paths
}

pub fn only_keep_shortest_paths(paths: &Vec<PathC>) -> Vec<PathC> {
    let min_lens = paths.iter().map(|p| p.len()).min().unwrap();
    let paths: Vec<PathC> = paths
        .iter()
        .filter(|&p| p.len() == min_lens)
        .map(|p| p.clone())
        .collect();
    // paths.sort();
    paths
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let codes = input.lines().map(|line| line.chars().collect()).collect();
        Puzzle { codes }
    }

    pub fn solve_p1(&self, nb_robots: usize) -> usize {
        let mut sum = 0;
        for code in &self.codes {
            println!("[DDA] day21:: trying to type code: {:?}", code);
            let mut paths = get_paths_for_nums(code);
            paths = only_keep_shortest_paths(&paths);
            paths = only_keep_good_paths(&paths);

            // paths = vec!["<v".chars().collect::<Vec<char>>()];
            for i in 0..nb_robots {
                paths = paths
                    .iter()
                    .flat_map(get_paths_for_keys)
                    .collect::<Vec<Vec<char>>>();
                paths = only_keep_shortest_paths(&paths);
                paths = only_keep_good_paths(&paths);

                println!(
                    "[DDA] day21:: nb-paths after {} keypads {} (len = {})",
                    i + 1,
                    paths.len(),
                    paths[0].len()
                );
            }
            let code_string: String = code.into_iter().collect();
            let code_str = &code_string[..3];
            let code_i32 = tousize(code_str);
            // println!("[DDA] day21:: {} x {}", code_i32, paths[0].len());
            sum += paths[0].len() * code_i32;
            // break;
        }
        sum
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve_p1(2)
}

fn p2(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve_p1(25)
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
    time_it(p1, "p1", "data/21_input.txt");
    // time_it(p2, "p2", "data/21_sample.txt");
    // time_it(p2, "p2", "data/21_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/21_sample.txt"), 126384);
        assert_eq!(run_it(p1, "data/21_input.txt"), 188398);
        // assert_eq!(run_it(p2, "data/21_sample.txt"), 16);
        // assert_eq!(run_it(p2, "data/21_input.txt"), 595975512785325);
    }
}
