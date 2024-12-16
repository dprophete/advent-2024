use std::{
    fmt::{self, Display},
    fs,
    time::{Duration, Instant},
};

//--------------------------------------------------------------------------------
// number parsing
//--------------------------------------------------------------------------------

pub fn toi64(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

pub fn tou64(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

pub fn toi32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

pub fn toi128(s: &str) -> i128 {
    s.parse::<i128>().unwrap()
}

pub fn tou32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

pub fn c_tou32(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

//--------------------------------------------------------------------------------
// v2
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

impl V2 {
    pub const UP: V2 = V2 { x: 0, y: -1 };
    pub const DOWN: V2 = V2 { x: 0, y: 1 };
    pub const LEFT: V2 = V2 { x: -1, y: 0 };
    pub const RIGHT: V2 = V2 { x: 1, y: 0 };

    pub fn new(x: i32, y: i32) -> V2 {
        V2 { x, y }
    }

    pub fn from_vec(v: &[i32]) -> V2 {
        V2::new(v[0], v[1])
    }

    pub fn scale(&self, n: i32) -> V2 {
        V2::new(self.x * n, self.y * n)
    }

    pub fn add(&self, other: &V2) -> V2 {
        V2::new(self.x + other.x, self.y + other.y)
    }

    pub fn modulo(&self, other: &V2) -> V2 {
        V2::new(self.x.rem_euclid(other.x), self.y.rem_euclid(other.y))
    }

    pub fn sub(&self, other: &V2) -> V2 {
        V2::new(self.x - other.x, self.y - other.y)
    }

    pub fn move_to_dir(&self, dir: &Dir) -> V2 {
        self.add(dir.to_v2())
    }

    pub fn neighbors(&self, pos: &V2) -> Vec<V2> {
        [V2::UP, V2::DOWN, V2::LEFT, V2::RIGHT]
            .iter()
            .map(|dir| pos.add(dir))
            .collect()
    }
}

//--------------------------------------------------------------------------------
// v3
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct V3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

//--------------------------------------------------------------------------------
// direction
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn rot_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    pub fn rot_left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_v2(&self) -> &V2 {
        match self {
            Dir::Up => &V2::UP,
            Dir::Down => &V2::DOWN,
            Dir::Left => &V2::LEFT,
            Dir::Right => &V2::RIGHT,
        }
    }
}

//--------------------------------------------------------------------------------
// matrix
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    pub matrix: Vec<Vec<T>>,
    pub width: i32,
    pub height: i32,
}

// base matrix
impl<T: Clone + PartialEq> Matrix<T> {
    pub fn from_vec(matrix: Vec<Vec<T>>) -> Matrix<T> {
        let height = matrix.len() as i32;
        let width = matrix[0].len() as i32;
        Matrix {
            matrix,
            width,
            height,
        }
    }

    pub fn find_first(&self, value: T) -> Option<V2> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.matrix[y as usize][x as usize] == value {
                    return Some(V2::new(x, y));
                }
            }
        }
        None
    }

    pub fn find_all(&self, value: T) -> Vec<V2> {
        let mut res = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.matrix[y as usize][x as usize] == value {
                    res.push(V2::new(x, y));
                }
            }
        }
        res
    }

    pub fn is_in(&self, pos: &V2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }

    // return char at x, y or '.' if out of bounds
    pub fn get(&self, pos: &V2) -> Option<T> {
        if self.is_in(pos) {
            Some(self.matrix[pos.y as usize][pos.x as usize].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: &V2, value: T) -> &mut Self {
        if self.is_in(pos) {
            self.matrix[pos.y as usize][pos.x as usize] = value;
        }
        self
    }

    pub fn from_str(content: &str, convert: fn(char) -> T) -> Matrix<T> {
        let matrix: Vec<Vec<T>> = content
            .lines()
            .map(|line| line.chars().map(convert).collect())
            .collect();
        Matrix::from_vec(matrix)
    }

    pub fn neighbors(&self, pos: &V2) -> Vec<V2> {
        pos.neighbors(pos)
            .iter()
            .filter(|nx| self.is_in(nx))
            .cloned()
            .collect()
    }
}

impl<T: Display + Clone + PartialEq> Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(value) = self.get(&V2::new(x, y)) {
                    write!(f, "{}", value)?;
                } else {
                    write!(f, "*")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

//--------------------------------------------------------------------------------
// formatting
//--------------------------------------------------------------------------------

pub fn fmt_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_micros() > 0 {
        format!("{}µs", duration.as_micros())
    } else {
        format!("{}ns", duration.as_nanos())
    }
}

//--------------------------------------------------------------------------------
// misc
//--------------------------------------------------------------------------------

// false -> 0, true -> 1
pub fn bool_to_u32(b: bool) -> u32 {
    b as u32
}

pub fn pp_day(txt: &str) {
    println!("\n#### {}", txt);
}

// time p1/p2 function with the content of the file
pub fn time_it<R: fmt::Display>(p: fn(&str) -> R, p_str: &str, file: &str) {
    let start = Instant::now();
    let res = run_it(p, file);
    let duration = start.elapsed();
    println!(
        "[{}] {} : {} -> {}",
        fmt_duration(duration),
        p_str,
        file,
        res
    );
}

// run p1/p2 function with the content of the file
pub fn run_it<R>(p: fn(&str) -> R, file: &str) -> R {
    let input = fs::read_to_string(file).expect("cannot read sample file");
    p(input.trim_end())
}
