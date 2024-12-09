use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

// part1 matrix extension
impl Matrix {
    // check if MAS is at (x, y) in the dir (dx, dy)
    fn is_mas_in_dir(&self, pos: V2, dir: V2) -> u32 {
        bool_to_u32(
            self.get(&pos.add(&dir)) == Some('M')
                && self.get(&pos.add(&dir.times(2))) == Some('A')
                && self.get(&pos.add(&dir.times(3))) == Some('S'),
        )
    }

    fn nb_xmas_at_point(&self, pos: V2) -> u32 {
        // short circuit
        if self.get(&pos) != Some('X') {
            return 0;
        }
        // we check clockwise, starting from the top
        self.is_mas_in_dir(pos, V2::new(0, -1))
            + self.is_mas_in_dir(pos, V2::new(1, -1))
            + self.is_mas_in_dir(pos, V2::new(1, 0))
            + self.is_mas_in_dir(pos, V2::new(1, 1))
            + self.is_mas_in_dir(pos, V2::new(0, 1))
            + self.is_mas_in_dir(pos, V2::new(-1, 1))
            + self.is_mas_in_dir(pos, V2::new(-1, 0))
            + self.is_mas_in_dir(pos, V2::new(-1, -1))
    }
}

fn p1(input: &str) -> u32 {
    let matrix = Matrix::from_str(input);
    let size: i32 = matrix.size;

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size {
            sum += matrix.nb_xmas_at_point(V2::new(x, y))
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// part2 matrix extension
impl Matrix {
    // check if M-S is around (x, y) in the dir (dx, dy)
    fn is_ms_in_dir(&self, pos: V2, dir: V2) -> bool {
        self.get(&pos.add(&dir)) == Some('M') && self.get(&pos.sub(&dir)) == Some('S')
    }

    fn is_x_dash_mas_at_point(&self, pos: V2) -> bool {
        // short circuit
        if self.get(&pos) != Some('A') {
            return false;
        }
        (self.is_ms_in_dir(pos, V2::new(1, 1)) || self.is_ms_in_dir(pos, V2::new(-1, -1)))
            && (self.is_ms_in_dir(pos, V2::new(-1, 1)) || self.is_ms_in_dir(pos, V2::new(1, -1)))
    }
}

fn p2(input: &str) -> u32 {
    let matrix = Matrix::from_str(input);
    let size: i32 = matrix.size;

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size {
            sum += bool_to_u32(matrix.is_x_dash_mas_at_point(V2::new(x, y)))
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// run
//--------------------------------------------------------------------------------

pub fn run() {
    time_it(p1, "data/04_sample.txt");
    time_it(p1, "data/04_input.txt");
    time_it(p2, "data/04_sample.txt");
    time_it(p2, "data/04_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(run_it(p1, "data/04_sample.txt"), 18);
    }

    #[test]
    fn test_p2() {
        assert_eq!(run_it(p2, "data/04_sample.txt"), 9);
    }
}
