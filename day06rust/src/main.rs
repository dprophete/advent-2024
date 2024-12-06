#![allow(dead_code)]

use std::fmt::{self, Display};
use std::fs;

type V2 = (i32, i32);

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

fn move1((x, y): V2, (vx, vy): V2) -> V2 {
    (x + vx, y + vy)
}

fn rot_right((vx, vy): V2) -> V2 {
    match (vx, vy) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("invalid direction"),
    }
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let mut matrix = Matrix::from_file_content(file_content.as_str());

    let mut pos = matrix.find_start();
    matrix.set(pos, 'X');
    let mut dir = (0, -1);
    let mut sum = 1;
    loop {
        let nx = move1(pos, dir);
        match matrix.get(nx) {
            None => break,
            Some('#') => dir = rot_right(dir),
            Some('X') => pos = nx,
            Some('.') => {
                pos = nx;
                matrix.set(pos, 'X');
                sum += 1;
            }
            c => panic!("invalid char {:?}", c),
        }
    }

    println!("p1 steps for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let mut matrix = Matrix::from_file_content(file_content.as_str());

    let mut pos = matrix.find_start();
    matrix.set(pos, 'X');
    let mut dir = (0, -1);
    let mut sum = 1;
    loop {
        let nx = move1(pos, dir);
        match matrix.get(nx) {
            None => break,
            Some('#') => dir = rot_right(dir),
            Some('X') => pos = nx,
            Some('.') => {
                pos = nx;
                matrix.set(pos, 'X');
                sum += 1;
            }
            c => panic!("invalid char {:?}", c),
        }
    }

    println!("p2 steps for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    p2("sample.txt");
    // p2("input.txt");
}
