use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::utils::*;

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
static DIR_KP: Lazy<Matrix<char>> = Lazy::new(|| Matrix::from_vec(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]));

// num shortest paths (from, to) -> list of shortest paths
static NUM_SP: Lazy<HashMap<(char, char), PathC>> = Lazy::new(|| compute_matrix_shortest_paths(&NUM_KP));

// dir shortest paths (from, to) -> list of shortest paths
static DIR_SP: Lazy<HashMap<(char, char), PathC>> = Lazy::new(|| compute_matrix_shortest_paths(&DIR_KP));

//--------------------------------------------------------------------------------
// preparing shortest paths for dir and num keypads
// we want to compute the shortest paths between all pairs of keys
//
// note: this is way way too complicated for what it does
// I might as well have hardcoded the paths
//--------------------------------------------------------------------------------

impl Dir {
    fn to_dir_keypad(self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        }
    }
}

fn path_dir_to_path_char(path: &PathD) -> PathC {
    path.iter().map(|d| d.to_dir_keypad()).collect()
}

fn compute_matrix_shortest_paths(matrix: &Matrix<char>) -> HashMap<(char, char), PathC> {
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
    // we need to get down to 1 path per pair
    let mut cost_at_pos2 = HashMap::new();
    for (&(k1, k2), v) in &cost_at_pos {
        if k1 == k2 {
            cost_at_pos2.insert((k1, k2), vec!['A']);
        } else {
            let v = keep_one(v);
            cost_at_pos2.insert((k1, k2), v.clone());
        }
    }
    cost_at_pos2
}

// let's iterate over the paths and count how many times we change letter
pub fn score_change_dir(path: &PathC) -> usize {
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

pub fn score_dist_to_a(path: &PathC) -> PathC {
    path.iter()
        .map(|c| match c {
            'A' => '0',
            '^' => '1',
            '>' => '1',
            'v' => '2',
            '<' => '3',
            _ => panic!("unexpected char {}", c),
        })
        .collect()
}

pub fn keep_one(paths: &[PathC]) -> PathC {
    let min_lens = paths.iter().map(|p| p.len()).min().unwrap();
    let min_score = paths.iter().map(score_change_dir).min().unwrap();

    let mut paths: Vec<&PathC> = paths
        .iter()
        .filter(|&p| p.len() == min_lens)
        .filter(|&p| score_change_dir(p) == min_score)
        .collect();
    paths.sort_by_key(|&p| score_dist_to_a(p));
    let &lst = paths.last().unwrap();
    lst.to_vec()
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    codes: Vec<Vec<char>>,
}

fn pp_path(path: &PathC) {
    for i in path {
        print!("{}", i);
    }
    println!();
}

pub fn get_path_for_nums(code: &Vec<char>) -> PathC {
    let mut pos_arm = 'A';
    let mut new_paths = vec![];
    for &c in code {
        // need to go from pos_arm to c
        let path_to_c = NUM_SP.get(&(pos_arm, c)).unwrap();
        new_paths.push(path_to_c.clone());
        pos_arm = c;
    }
    new_paths.concat()
}

pub fn shortest_len(pos_arm: char, c: char, level: usize, cache: &mut HashMap<(char, char, usize), usize>) -> usize {
    if level == 1 {
        // this is us controllig the closest robot
        return DIR_SP.get(&(pos_arm, c)).unwrap().len();
    }
    if let Some(&len) = cache.get(&(pos_arm, c, level)) {
        return len;
    }

    let mut len = 0;
    let path_to_c = DIR_SP.get(&(pos_arm, c)).unwrap();
    let mut local_pos_arm = 'A';
    for &tgt in path_to_c {
        len += shortest_len(local_pos_arm, tgt, level - 1, cache);
        local_pos_arm = tgt;
    }
    cache.insert((pos_arm, c, level), len);
    len
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let codes = input.lines().map(|line| line.chars().collect()).collect();
        Puzzle { codes }
    }

    pub fn solve(&self, nb_robots: usize) -> usize {
        let mut sum = 0;
        let mut cache: HashMap<(char, char, usize), usize> = HashMap::new();
        for code in &self.codes {
            // println!("[DDA] day21:: trying to type code: {:?}", code);

            let code_string: String = code.iter().filter(|c| c.is_ascii_digit()).collect();
            let code_i32 = tousize(&code_string);

            // this is the path we need the final robot to type on the door
            let path = get_path_for_nums(code);

            let mut pos_arm = 'A';
            for c in path {
                sum += code_i32 * shortest_len(pos_arm, c, nb_robots, &mut cache);
                pos_arm = c;
            }
        }
        sum
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve(2)
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve(25)
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day21: Keypad Conundrum");
    time_it(p1, "p1", "data/21_sample.txt");
    time_it(p1, "p1", "data/21_input.txt");
    time_it(p2, "p2", "data/21_sample.txt");
    time_it(p2, "p2", "data/21_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/21_sample.txt"), 126384);
        assert_eq!(run_it(p1, "data/21_input.txt"), 188398);
        assert_eq!(run_it(p2, "data/21_sample.txt"), 154115708116294);
        assert_eq!(run_it(p2, "data/21_input.txt"), 230049027535970);
    }
}
