use std::{collections::HashMap, convert::identity};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn p1(input: &str) -> i32 {
    let mut matrix = Matrix::from_str(input, identity);
    let start = matrix.find_first('S').unwrap();
    let finish = matrix.find_first('E').unwrap();
    matrix.set(&start, '.');
    matrix.set(&finish, '.');

    let mut visited = HashMap::new();

    let mut to_explore = vec![(start, Dir::Right, 0)];
    while let Some((pos, dir, score)) = to_explore.pop() {
        if matrix.get(&pos) != Some('.') {
            // we hit a wall
            continue;
        }

        if let Some(prev_score) = visited.get(&pos) {
            if score >= *prev_score {
                // been there already with a lower score...
                continue;
            }
        }
        visited.insert(pos, score);

        let nx = pos.add(&dir.to_v2());
        to_explore.push((nx, dir, score + 1));

        let left_dir = dir.rot_left();
        let nx_left = pos.add(&left_dir.to_v2());
        to_explore.push((nx_left, left_dir, score + 1 + 1000));

        let right_dir = dir.rot_right();
        let nx_right = pos.add(&right_dir.to_v2());
        to_explore.push((nx_right, right_dir, score + 1 + 1000));
    }

    *visited.get(&finish).unwrap()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day16: Warehouse Woes");
    time_it(p1, "p1", "data/16_sample.txt");
    time_it(p1, "p1", "data/16_sample2.txt");
    time_it(p1, "p1", "data/16_input.txt");
    // time_it(p2, "p2", "data/16_sample.txt");
    // time_it(p2, "p2", "data/16_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/16_sample.txt"), 7036);
        assert_eq!(run_it(p1, "data/16_sample2.txt"), 11048);
    }
}
