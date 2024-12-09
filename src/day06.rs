use crate::utils::*;
use std::collections::HashSet;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn find_start(matrix: &Matrix) -> V2 {
    for y in 0..matrix.size {
        for x in 0..matrix.size {
            if matrix.get(&V2::new(x, y)) == Some('^') {
                return V2::new(x, y);
            }
        }
    }
    V2::new(-1, -1)
}

fn p1(input: &str) -> i32 {
    let mut matrix = Matrix::from_str(input);

    let mut pos = find_start(&matrix);
    let mut dir = Direction::Up;
    let mut sum = 1;
    loop {
        let nx = pos.move_to_dir(&dir);
        match matrix.get(&nx) {
            None => break,
            Some('#') => dir = dir.rot_right(),
            Some('.') => {
                pos = nx;
                matrix.set(&pos, 'X'); // so we can remember where we've been here
                sum += 1;
            }
            _ => pos = nx,
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn is_in_loop(matrix: &Matrix, start: V2) -> bool {
    let mut pos = start;
    let mut times_at_pos = HashSet::new();
    let mut dir = Direction::Up;
    loop {
        let nx = pos.move_to_dir(&dir);
        match matrix.get(&nx) {
            None => return false,
            Some('#') | Some('O') => dir = dir.rot_right(),
            _ => {
                if times_at_pos.contains(&(pos, dir)) {
                    return true;
                }
                times_at_pos.insert((pos, dir));
                pos = nx;
            }
        }
    }
}

fn p2(input: &str) -> i32 {
    let mut matrix = Matrix::from_str(input);

    let start = find_start(&matrix);
    let mut sum = 0;

    let mut dir = Direction::Up;
    let mut pos = start;

    loop {
        let nx = pos.move_to_dir(&dir);
        match matrix.get(&nx) {
            None => break,
            Some('#') => dir = dir.rot_right(),
            Some('.') => {
                // let's try to put an obstacle here and see if we are in a loop...
                pos = nx;
                matrix.set(&pos, 'O');
                if is_in_loop(&matrix, start) {
                    sum += 1;
                }
                matrix.set(&pos, 'X'); // so we can remember where we've been here
            }
            _ => pos = nx,
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    time_it(p1, "data/06_sample.txt");
    time_it(p1, "data/06_input.txt");
    time_it(p2, "data/06_sample.txt");
    // time_it(p2, "data/06_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(run_it(p1, "data/06_sample.txt"), 41);
    }

    #[test]
    fn test_p2() {
        assert_eq!(run_it(p2, "data/06_sample.txt"), 6);
    }
}
