use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    patterns: Vec<Vec<char>>,
    designs: Vec<Vec<char>>,
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let (patterns, designs) = input.split_once("\n\n").unwrap();
        let patterns = patterns
            .split(", ")
            .map(|line| line.chars().collect())
            .collect();
        let designs = designs.lines().map(|line| line.chars().collect()).collect();

        Puzzle { patterns, designs }
    }

    pub fn solve_for_design(&self, design: &Vec<char>) -> bool {
        if design.is_empty() {
            return true;
        }
        for p in &self.patterns {
            if design.starts_with(p) {
                if self.solve_for_design(&design[p.len()..].to_vec()) {
                    return true;
                }
            }
        }
        false
    }
}

fn p1(input: &str) -> u32 {
    let puzzle = Puzzle::from_str(input);
    let mut sum = 0;
    for d in &puzzle.designs {
        sum += bool_to_u32(puzzle.solve_for_design(&d));
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> V2 {
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day19: Linen Layout");
    time_it(p1, "p1", "data/19_sample.txt");
    time_it(p1, "p2", "data/19_input.txt");
    // time_it(p2, "p2", "data/19_sample.txt");
    // time_it(p2, "p2", "data/19_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/19_sample.txt"), 6);
        assert_eq!(run_it(p1, "data/19_input.txt"), 242);
        // assert_eq!(run_it(p2, "data/19_sample.txt"), V2::new(6, 1));
        // assert_eq!(run_it(p2, "data/19_input.txt"), V2::new(28, 56));
    }
}
