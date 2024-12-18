use std::collections::HashMap;

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

        Memory {
            width,
            height,
            bytes,
        }
    }

    // pub fn fall(&mut self) {
    //     for v in self.bytes.iter_mut() {
    //         v.y += 1;
    //     }
    //     let height = self.height as i32;
    //     self.bytes = self
    //         .bytes
    //         .clone()
    //         .into_iter()
    //         .filter(|v| v.y < height)
    //         .collect();
    // }

    pub fn to_matrix(&self) -> Matrix<char> {
        let mut matrix = Matrix::with_size(self.width, self.height, '.');

        for v in &self.bytes {
            matrix.set(v, '#');
        }

        matrix
    }

    pub fn pp(&self) {
        let matrix = self.to_matrix();
        println!("{}", matrix);
    }

    pub fn steps_to_escape(&self) -> i32 {
        let matrix = self.to_matrix();
        let mut visited = HashMap::new();
        let mut to_explore = vec![(V2::new(0, 0), 0)];
        let exit = V2::new(self.width as i32 - 1, self.height as i32 - 1);

        let mut max_len = i32::MAX;
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
}

fn p1(input: &str, take: usize) -> i32 {
    let memory = Memory::from_str(input, take);
    memory.steps_to_escape()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day18: RAM Run");
    time_it(|input| p1(input, 12), "p1", "data/18_sample.txt");
    time_it(|input| p1(input, 1024), "p1", "data/18_input.txt");
    // time_it(p1, "p1", "data/18_input.txt");
    // time_it(p2, "p2", "data/18_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(|input| p1(input, 12), "data/18_sample.txt"), 22);
        assert_eq!(run_it(|input| p1(input, 12), "data/18_input.txt"), 280);
    }
}
