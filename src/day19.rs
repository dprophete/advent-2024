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
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    println!("[DDA] day19::puzzle {:?}", puzzle);
    // memory.nb_steps_to_escape()
    10
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
    // time_it(p1, "p2", "data/19_input.txt");
    // time_it(p2, "p2", "data/19_sample.txt");
    // time_it(p2, "p2", "data/19_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/19_sample.txt"), 6);
        // assert_eq!(run_it(p2, "data/19_sample.txt"), V2::new(6, 1));
        // assert_eq!(run_it(p2, "data/19_input.txt"), V2::new(28, 56));
    }
}
