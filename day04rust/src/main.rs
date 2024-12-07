#![allow(dead_code)]

use std::fs;

// false -> 0, true -> 1
fn bool_to_u16(b: bool) -> u16 {
    b as u16
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
    fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return '.';
        }
        self.matrix[y as usize][x as usize]
    }
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

// part1 matrix extension
impl Matrix {
    // check if MAS is at (x, y) in the dir (dx, dy)
    fn is_mas_in_dir(&self, x: i32, y: i32, dx: i32, dy: i32) -> u16 {
        bool_to_u16(
            self.get(x + dx, y + dy) == 'M'
                && self.get(x + 2 * dx, y + 2 * dy) == 'A'
                && self.get(x + 3 * dx, y + 3 * dy) == 'S',
        )
    }

    fn nb_xmas_at_point(&self, x: i32, y: i32) -> u16 {
        // short circuit
        if self.get(x, y) != 'X' {
            return 0;
        }
        // we check clockwise, starting from the top
        self.is_mas_in_dir(x, y, 0, -1)
            + self.is_mas_in_dir(x, y, 1, -1)
            + self.is_mas_in_dir(x, y, 1, 0)
            + self.is_mas_in_dir(x, y, 1, 1)
            + self.is_mas_in_dir(x, y, 0, 1)
            + self.is_mas_in_dir(x, y, -1, 1)
            + self.is_mas_in_dir(x, y, -1, 0)
            + self.is_mas_in_dir(x, y, -1, -1)
    }
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let matrix = Matrix::from_file_content(file_content.as_str());
    let size: i32 = matrix.size;

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size {
            sum += matrix.nb_xmas_at_point(x, y)
        }
    }

    println!("p1 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// part2 matrix extension
impl Matrix {
    // check if M-S is around (x, y) in the dir (dx, dy)
    fn is_ms_in_dir(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        self.get(x - dx, y - dy) == 'M' && self.get(x + dx, y + dy) == 'S'
    }

    fn is_x_dash_mas_at_point(&self, x: i32, y: i32) -> bool {
        // short circuit
        if self.get(x, y) != 'A' {
            return false;
        }
        (self.is_ms_in_dir(x, y, 1, 1) || self.is_ms_in_dir(x, y, -1, -1))
            && (self.is_ms_in_dir(x, y, -1, 1) || self.is_ms_in_dir(x, y, 1, -1))
    }
}

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let matrix = Matrix::from_file_content(file_content.as_str());
    let size: i32 = matrix.size;

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size {
            sum += bool_to_u16(matrix.is_x_dash_mas_at_point(x, y))
        }
    }

    println!("p1 sum for {} -> {}", input, sum);
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
