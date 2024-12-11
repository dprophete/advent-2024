/// hiking the trails
use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

impl Matrix<i32> {
    // find all the starting points
    fn get_starts(&self) -> Vec<V2> {
        let mut starts = vec![];
        for y in 0..self.size {
            for x in 0..self.size {
                if self.get(&V2::new(x, y)) == Some(0) {
                    starts.push(V2::new(x, y));
                }
            }
        }
        starts
    }

    // for a given pos, find the next candidate positions
    fn get_nxs(&self, pos: &V2) -> Vec<V2> {
        let mut res = vec![];
        let val = self.get(pos).unwrap();
        for dir in [V2::UP, V2::DOWN, V2::LEFT, V2::RIGHT] {
            let nx = pos.add(&dir);
            match self.get(&nx) {
                Some(v) if v == val + 1 => {
                    res.push(nx);
                }
                _ => {}
            }
        }
        res
    }

    fn nb_trails_for_start(&self, start: V2, with_ratings: bool) -> usize {
        let mut matrix = self.clone();
        let mut nb_trails = 0;
        let mut to_explore = vec![start];
        while let Some(pos) = to_explore.pop() {
            let nxs = matrix.get_nxs(&pos);
            for nx in nxs {
                if matrix.get(&nx) == Some(9) {
                    if !with_ratings {
                        // make sure we don't reach that endpoint again
                        matrix.set(&nx, -1);
                    }
                    nb_trails += 1;
                } else {
                    to_explore.push(nx);
                }
            }
        }
        nb_trails
    }
}

fn convert(c: char) -> i32 {
    match c {
        '.' => -1,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn p1(input: &str) -> usize {
    let matrix = Matrix::from_str(input, convert);

    let mut nb_trails = 0;
    for start in &matrix.get_starts() {
        nb_trails += matrix.nb_trails_for_start(*start, false);
    }
    nb_trails
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let matrix = Matrix::from_str(input, convert);

    let mut nb_trails = 0;
    for start in &matrix.get_starts() {
        nb_trails += matrix.nb_trails_for_start(*start, true);
    }
    nb_trails
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day10: hiking the trails");
    time_it(p1, "p1", "data/10_sample1a.txt");
    time_it(p1, "p1", "data/10_sample1b.txt");
    time_it(p1, "p1", "data/10_sample1c.txt");
    time_it(p1, "p1", "data/10_sample1d.txt");
    time_it(p1, "p1", "data/10_sample1e.txt");
    time_it(p1, "p1", "data/10_input.txt");
    time_it(p2, "p2", "data/10_sample2a.txt");
    time_it(p2, "p2", "data/10_sample2b.txt");
    time_it(p2, "p2", "data/10_sample2c.txt");
    time_it(p2, "p2", "data/10_sample1e.txt");
    time_it(p2, "p2", "data/10_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/10_sample1a.txt"), 1);
        assert_eq!(run_it(p1, "data/10_sample1b.txt"), 2);
        assert_eq!(run_it(p1, "data/10_sample1c.txt"), 4);
        assert_eq!(run_it(p1, "data/10_sample1d.txt"), 3);
        assert_eq!(run_it(p1, "data/10_sample1e.txt"), 36);
        assert_eq!(run_it(p2, "data/10_sample2a.txt"), 3);
        assert_eq!(run_it(p2, "data/10_sample2b.txt"), 13);
        assert_eq!(run_it(p2, "data/10_sample2c.txt"), 227);
        assert_eq!(run_it(p2, "data/10_sample1e.txt"), 81);
    }
}
