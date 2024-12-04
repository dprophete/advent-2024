#![feature(array_chunks)]
#![allow(dead_code)]

use std::fs;

//--------------------------------------------------------------------------------
// p1
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

    // return char at x, y or '.' if out of bounds
    fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return '.';
        }
        return self.matrix[y as usize][x as usize];
    }
}

// xmas matrix extension
impl Matrix {
    // return 4 chars in a given direction
    fn chars_in_dir(&self, x: i32, y: i32, dx: i32, dy: i32) -> (char, char, char) {
        (
            self.get(x + dx, y + dy),
            self.get(x + 2 * dx, y + 2 * dy),
            self.get(x + 3 * dx, y + 3 * dy),
        )
    }

    fn check_in_dir(&self, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
        if is_mas(self.chars_in_dir(x, y, dx, dy)) {
            1
        } else {
            0
        }
    }

    fn check_at_point(&self, x: i32, y: i32) -> i32 {
        // short circuit
        if self.get(x, y) != 'X' {
            return 0;
        }
        // we check clockwise, starting from the top
        self.check_in_dir(x, y, 0, -1)
            + self.check_in_dir(x, y, 1, -1)
            + self.check_in_dir(x, y, 1, 0)
            + self.check_in_dir(x, y, 1, 1)
            + self.check_in_dir(x, y, 0, 1)
            + self.check_in_dir(x, y, -1, 1)
            + self.check_in_dir(x, y, -1, 0)
            + self.check_in_dir(x, y, -1, -1)
    }
}

// check if 3 chars are MAS
fn is_mas((m, a, s): (char, char, char)) -> bool {
    m == 'M' && a == 'A' && s == 'S'
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let chars: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let matrix = Matrix::new(chars);
    let size: i32 = matrix.size;

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size {
            sum += matrix.check_at_point(x, y)
        }
    }

    println!("p1 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) {
//     let file_content = fs::read_to_string(input).expect("cannot read sample file");
//
//     let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|do\(\)|don't\(\)").unwrap();
//
//     let mut enabled = true;
//     let mut sum = 0;
//     re.captures_iter(&file_content).for_each(|caps| {
//         if caps[0].to_string() == "do()" {
//             enabled = true
//         } else if caps[0].to_string() == "don't()" {
//             enabled = false
//         } else if enabled {
//             let n1 = caps[1].parse::<u32>().unwrap();
//             let n2 = caps[2].parse::<u32>().unwrap();
//             sum += n1 * n2
//         }
//     });
//
//     println!("p2 sum for {} -> {}", input, sum);
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    // p2("sample2.txt");
    // p2("input.txt");
}
