#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs;

type V2 = (i32, i32);

//--------------------------------------------------------------------------------
// direction
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn rot_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn to_v2(&self) -> V2 {
        match self {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        }
    }
}

//--------------------------------------------------------------------------------
// matrix
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Matrix {
    matrix: Vec<Vec<char>>,
    size: i32,
}

// base matrix
impl Matrix {
    fn new(matrix: Vec<Vec<char>>) -> Matrix {
        let size = matrix.len() as i32;
        Matrix { matrix, size }
    }

    fn from_file_content(file_content: &str) -> Matrix {
        let matrix: Vec<Vec<char>> = file_content
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Matrix::new(matrix)
    }

    // return char at x, y or '.' if out of bounds
    fn get(&self, (x, y): V2) -> Option<char> {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return None;
        }
        return Some(self.matrix[y as usize][x as usize]);
    }

    fn set(&mut self, (x, y): V2, c: char) -> &mut Self {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return self;
        }
        self.matrix[y as usize][x as usize] = c;
        self
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "{}", self.get((x, y)).unwrap_or('*'))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

impl Matrix {
    fn find_start(&self) -> V2 {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.get((x, y)) == Some('^') {
                    return (x, y);
                }
            }
        }
        return (-1, -1);
    }
}

fn move1((x, y): V2, dir: &Direction) -> V2 {
    let (vx, vy) = dir.to_v2();
    (x + vx, y + vy)
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let mut matrix = Matrix::from_file_content(file_content.as_str());

    let mut pos = matrix.find_start();
    let mut dir = Direction::UP;
    let mut sum = 1;
    loop {
        let nx = move1(pos, &dir);
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

    println!("p1 steps for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// we figure out we are in a loop when we visit the same position 5 times
// (if it's 5 times, we are sure that for 2 of these we were going intot the same direction)
// I know, I know, it's not optimized but it's rust ;-)
fn is_in_loop(matrix: &Matrix, start: V2) -> bool {
    let mut pos = start;
    let mut times_at_pos = HashMap::new();
    let mut dir = Direction::UP;
    loop {
        let nx = move1(pos, &dir);
        match matrix.get(nx) {
            None => return false,
            Some('#') | Some('O') => dir = dir.rot_right(),
            _ => {
                let current = *times_at_pos.get(&pos).unwrap_or(&0);
                if current == 4 {
                    return true;
                }
                times_at_pos.insert(pos, current + 1);
                pos = nx;
            }
        }
    }
}

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let mut matrix = Matrix::from_file_content(file_content.as_str());

    let start = matrix.find_start();
    let mut sum = 0;

    let mut dir = Direction::UP;
    let mut pos = start;

    loop {
        let nx = move1(pos, &dir);
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

    println!("p2 obstructions for {} -> {}", input, sum);
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