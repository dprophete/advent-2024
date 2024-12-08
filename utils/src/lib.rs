use std::{
    fmt::{self, Display},
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

//--------------------------------------------------------------------------------
// vectors
//--------------------------------------------------------------------------------

pub type V2 = (i32, i32);

pub type V3 = (i32, i32, i32);

//--------------------------------------------------------------------------------
// direction
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rot_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn to_v2(&self) -> V2 {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
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
    pub fn from_matrix(matrix: Vec<Vec<char>>) -> Matrix {
        let size = matrix.len() as i32;
        Matrix { matrix, size }
    }

    pub fn from_str(content: &str) -> Matrix {
        let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        Matrix::from_matrix(matrix)
    }

    // return char at x, y or '.' if out of bounds
    pub fn get(&self, (x, y): V2) -> Option<char> {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return None;
        }
        Some(self.matrix[y as usize][x as usize])
    }

    pub fn set(&mut self, (x, y): V2, c: char) -> &mut Self {
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
pub fn bool_to_u16(b: bool) -> u16 {
    b as u16
}
