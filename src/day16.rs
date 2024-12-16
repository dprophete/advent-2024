use std::{collections::HashSet, convert::identity};

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

    let mut min_score = i32::MAX;

    let mut visited = HashSet::new();
    let mut to_explore = vec![(start, Dir::Right, 0)];
    while let Some((pos, dir, score)) = to_explore.pop() {
        if visited.contains(&pos) {
            // been there already
            continue;
        }
        if matrix.get(&pos) != Some('.') {
            // we hit a wall
            continue;
        }
        if pos == finish {
            // we reached the end
            if score < min_score {
                min_score = score;
            }
            continue;
        }

        visited.insert(pos);
        to_explore.push((pos.add(&dir.to_v2()), dir, score + 1));
        let left_dir = dir.rot_left();
        let right_dir = dir.rot_right();
        to_explore.push((pos.add(&left_dir.to_v2()), left_dir, score + 1 + 1000));
        to_explore.push((pos.add(&right_dir.to_v2()), right_dir, score + 1 + 1000));
    }

    min_score
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
    // time_it(p1, "p1", "data/16_input.txt");
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
