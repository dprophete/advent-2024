#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use utils::{fmt_t, Direction, Matrix, V2};

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn find_start(matrix: &Matrix) -> V2 {
    for y in 0..matrix.size {
        for x in 0..matrix.size {
            if matrix.get((x, y)) == Some('^') {
                return (x, y);
            }
        }
    }
    (-1, -1)
}

fn move_to_dir((x, y): V2, dir: &Direction) -> V2 {
    let (vx, vy) = dir.to_v2();
    (x + vx, y + vy)
}

fn p1(input: &str) {
    let start_t = Instant::now();
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let mut matrix = Matrix::from_str(&file_content);

    let mut pos = find_start(&matrix);
    let mut dir = Direction::Up;
    let mut sum = 1;
    loop {
        let nx = move_to_dir(pos, &dir);
        match matrix.get(nx) {
            None => break,
            Some('#') => dir = dir.rot_right(),
            Some('.') => {
                pos = nx;
                matrix.set(pos, 'X'); // so we can remember where we've been here
                sum += 1;
            }
            _ => pos = nx,
        }
    }

    println!("[{}] p1 {} -> {}", fmt_t(start_t), input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// we figure out we are in a loop when we visit the same position 5 times
// (if it's 5 times, we are sure that for 2 of these we were going into the same direction)
// I know, I know, it's not optimized but it's rust ;-)
fn is_in_loop(matrix: &Matrix, start: V2) -> bool {
    let mut pos = start;
    let mut times_at_pos = HashMap::new();
    let mut dir = Direction::Up;
    loop {
        let nx = move_to_dir(pos, &dir);
        match matrix.get(nx) {
            None => return false,
            Some('#') | Some('O') => dir = dir.rot_right(),
            _ => {
                let current = *times_at_pos.get(&pos).unwrap_or(&0);
                times_at_pos.insert(pos, current + 1);
                if current == 5 {
                    return true;
                }
                pos = nx;
            }
        }
    }
}

fn p2(input: &str) {
    let start_t = Instant::now();
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let mut matrix = Matrix::from_str(&file_content);

    let start = find_start(&matrix);
    let mut sum = 0;

    let mut dir = Direction::Up;
    let mut pos = start;

    loop {
        let nx = move_to_dir(pos, &dir);
        match matrix.get(nx) {
            None => break,
            Some('#') => dir = dir.rot_right(),
            Some('.') => {
                // let's try to put an obstacle here and see if we are in a loop...
                pos = nx;
                matrix.set(pos, 'O');
                if is_in_loop(&matrix, start) {
                    sum += 1;
                }
                matrix.set(pos, 'X'); // so we can remember where we've been here
            }
            _ => pos = nx,
        }
    }

    println!("[{}] p2 {} -> {}", fmt_t(start_t), input, sum);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    p2("sample.txt");
    p2("input.txt");
}
