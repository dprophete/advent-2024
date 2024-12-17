use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn score_per_pos(input: &str) -> (i32, HashMap<(V2, Dir), i32>) {
    let mut min_score = i32::MAX;
    let mut matrix = Matrix::from_str(input, identity);
    let start = matrix.find_first('S').unwrap();
    let finish = matrix.find_first('E').unwrap();
    matrix.set(&start, '.');
    matrix.set(&finish, '.');

    // we are going to keep the score at each position
    // if we ever reach the same position with a higher score, we stop
    let mut visited = HashMap::new();

    let mut to_explore = vec![(start, Dir::Right, 0)];
    while let Some((pos, dir, score)) = to_explore.pop() {
        if matrix.get(&pos) != Some('.') {
            // we hit a wall
            continue;
        }

        if let Some(prev_score) = visited.get(&(pos, dir)) {
            if score >= *prev_score {
                // been there already with a lower score...
                continue;
            }
        }
        visited.insert((pos, dir), score);

        if pos == finish {
            if score < min_score {
                min_score = score;
            }
            continue;
        }

        let nx = pos.add(&dir.to_v2());
        to_explore.push((nx, dir, score + 1));

        let dir_left = dir.rot_left();
        let nx_left = pos.add(&dir_left.to_v2());
        to_explore.push((nx_left, dir_left, score + 1 + 1000));

        let dir_right = dir.rot_right();
        let nx_right = pos.add(&dir_right.to_v2());
        to_explore.push((nx_right, dir_right, score + 1 + 1000));
    }

    (min_score, visited)
}

fn p1(input: &str) -> i32 {
    let (score, _) = score_per_pos(input);
    score
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let (p1_score, visited) = score_per_pos(input);

    let mut matrix = Matrix::from_str(input, identity);
    let start = matrix.find_first('S').unwrap();
    let finish = matrix.find_first('E').unwrap();
    matrix.set(&start, '.');
    matrix.set(&finish, '.');

    let mut all_seats = HashSet::new();

    let mut to_explore = vec![(start, Dir::Right, 0, vec![])];
    while let Some((pos, dir, score, path)) = to_explore.pop() {
        if matrix.get(&pos) != Some('.') {
            // we hit a wall
            continue;
        }

        if let Some(prev_score) = visited.get(&(pos, dir)) {
            if score > *prev_score {
                continue;
            }
        }

        if pos == finish {
            if score == p1_score {
                all_seats.extend(path);
                continue;
            }
        }

        let mut path = path.clone();
        path.push(pos);

        let nx = pos.add(&dir.to_v2());
        to_explore.push((nx, dir, score + 1, path.clone()));

        let left_dir = dir.rot_left();
        let nx_left = pos.add(&left_dir.to_v2());
        to_explore.push((nx_left, left_dir, score + 1 + 1000, path.clone()));

        let right_dir = dir.rot_right();
        let nx_right = pos.add(&right_dir.to_v2());
        to_explore.push((nx_right, right_dir, score + 1 + 1000, path));
    }

    1 + all_seats.len()
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day16: Warehouse Woes");
    time_it(p1, "p1", "data/16_sample.txt");
    time_it(p1, "p1", "data/16_sample2.txt");
    // time_it(p1, "p1", "data/16_input.txt");
    time_it(p2, "p2", "data/16_sample.txt");
    time_it(p2, "p2", "data/16_sample2.txt");
    // time_it(p2, "p2", "data/16_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/16_sample.txt"), 7036);
        assert_eq!(run_it(p1, "data/16_sample2.txt"), 11048);
        assert_eq!(run_it(p2, "data/16_sample.txt"), 45);
        assert_eq!(run_it(p2, "data/16_sample2.txt"), 64);
    }
}
