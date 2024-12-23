use std::collections::{HashMap, HashSet};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Memory {
    width: usize,
    height: usize,
    bytes: Vec<V2>,
}

impl Memory {
    pub fn from_str(input: &str, take: usize) -> Memory {
        let bytes = input
            .lines()
            .take(take)
            .map(|line| line.split_once(",").unwrap())
            .map(|(x, y)| V2::new(toi32(x), toi32(y)))
            .collect::<Vec<_>>();
        let width = bytes.iter().map(|v| v.x).max().unwrap() as usize + 1;
        let height = bytes.iter().map(|v| v.x).max().unwrap() as usize + 1;

        Memory { width, height, bytes }
    }

    pub fn to_matrix(&self, take: usize) -> Matrix<char> {
        let mut matrix = Matrix::with_size(self.width, self.height, '.');

        let mut i = 0;
        for v in &self.bytes {
            i += 1;
            if i > take {
                break;
            }
            matrix.set(v, '#');
        }

        matrix
    }

    pub fn nb_steps_to_escape(&self) -> usize {
        let matrix = self.to_matrix(usize::MAX);
        let mut visited = HashMap::new();
        let mut to_explore = vec![(V2::new(0, 0), 0)];
        let exit = V2::new(self.width as i32 - 1, self.height as i32 - 1);

        let mut max_len = usize::MAX;
        while let Some((pos, len)) = to_explore.pop() {
            if pos == exit {
                if len < max_len {
                    max_len = len;
                }
                continue;
            }
            if let Some(current_max_at_pos) = visited.get(&pos) {
                if (*current_max_at_pos) <= len {
                    continue;
                }
            }
            visited.insert(pos, len);

            for nx in pos.neighbors() {
                if matrix.get(&nx) == Some('.') {
                    to_explore.push((nx, len + 1));
                }
            }
        }
        max_len
    }

    pub fn has_path_to_escape(&self, take: usize) -> bool {
        let matrix = self.to_matrix(take);
        let mut visited = HashSet::new();
        let mut to_explore = vec![V2::new(0, 0)];
        let exit = V2::new(self.width as i32 - 1, self.height as i32 - 1);

        while let Some(pos) = to_explore.pop() {
            if pos == exit {
                return true;
            }
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            for nx in pos.neighbors() {
                if matrix.get(&nx) == Some('.') {
                    to_explore.push(nx);
                }
            }
        }
        false
    }
}

fn p1(input: &str, take: usize) -> usize {
    let memory = Memory::from_str(input, take);
    memory.nb_steps_to_escape()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> V2 {
    let memory = Memory::from_str(input, usize::MAX);

    let mut take = 0;
    while memory.has_path_to_escape(take) {
        take += 1;
    }
    memory.bytes[take - 1]
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day18: RAM Run");
    time_it(|input| p1(input, 12), "p1", "data/18_sample.txt");
    time_it(|input| p1(input, 1024), "p1", "data/18_input.txt");
    time_it(p2, "p2", "data/18_sample.txt");
    time_it(p2, "p2", "data/18_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(|input| p1(input, 12), "data/18_sample.txt"), 22);
        assert_eq!(run_it(|input| p1(input, 1024), "data/18_input.txt"), 280);
        assert_eq!(run_it(p2, "data/18_sample.txt"), V2::new(6, 1));
        assert_eq!(run_it(p2, "data/18_input.txt"), V2::new(28, 56));
    }
}
