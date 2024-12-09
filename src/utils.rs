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

pub fn toi32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
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
    pub fn new(x: i32, y: i32) -> V2 {
        V2 { x, y }
    }

    pub fn from_vec(v: &[i32]) -> V2 {
        V2::new(v[0], v[1])
    }

    pub fn times(&self, n: i32) -> V2 {
        V2::new(self.x * n, self.y * n)
    }

    pub fn add(&self, other: &V2) -> V2 {
        V2::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &V2) -> V2 {
        V2::new(self.x - other.x, self.y - other.y)
    }

    pub fn move_to_dir(&self, dir: &Direction) -> V2 {
        self.add(dir.to_v2())
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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static V2_UP: V2 = V2 { x: 0, y: -1 };
static V2_DOWN: V2 = V2 { x: 0, y: 1 };
static V2_LEFT: V2 = V2 { x: -1, y: 0 };
static V2_RIGHT: V2 = V2 { x: 1, y: 0 };

impl Direction {
    pub fn rot_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rot_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_v2(&self) -> &V2 {
        match self {
            Direction::Up => &V2_UP,
            Direction::Down => &V2_DOWN,
            Direction::Left => &V2_LEFT,
            Direction::Right => &V2_RIGHT,
        }
    }
}

//--------------------------------------------------------------------------------
// matrix
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Matrix {
    pub matrix: Vec<Vec<char>>,
    pub size: i32,
}

// base matrix
impl Matrix {
    pub fn from_vec(matrix: Vec<Vec<char>>) -> Matrix {
        let size = matrix.len() as i32;
        Matrix { matrix, size }
    }

    pub fn from_str(content: &str) -> Matrix {
        let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        Matrix::from_vec(matrix)
    }

    pub fn is_in(&self, pos: &V2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size && pos.y < self.size
    }

    // return char at x, y or '.' if out of bounds
    pub fn get(&self, pos: &V2) -> Option<char> {
        if self.is_in(pos) {
            Some(self.matrix[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: &V2, c: char) -> &mut Self {
        if self.is_in(pos) {
            self.matrix[pos.y as usize][pos.x as usize] = c;
        }
        self
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "{}", self.get(&V2::new(x, y)).unwrap_or('*'))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

//--------------------------------------------------------------------------------
// formatting
//--------------------------------------------------------------------------------

pub fn fmt_t(instant: Instant) -> String {
    fmt_d(instant.elapsed())
}

pub fn fmt_d(duration: Duration) -> String {
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

// time p1/p2 function with the content of the file
pub fn time_it<R: fmt::Display>(p: fn(&str) -> R, file: &str) {
    let start_t = Instant::now();
    println!("[{}] {} -> {}", fmt_t(start_t), file, run_it(p, file));
}

// run p1/p2 function with the content of the file
pub fn run_it<R>(p: fn(&str) -> R, file: &str) -> R {
    let input = fs::read_to_string(file).expect("cannot read sample file");
    p(input.trim_end())
}
