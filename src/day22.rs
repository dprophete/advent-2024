use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    secret_numbers: Vec<u64>,
}

fn compute_nx(secret_number: u64) -> u64 {
    let result = secret_number * 64;
    // mix result into secret_number
    let secret_number = result ^ secret_number;
    // prune secret_number
    let secret_number = secret_number % 16777216;

    let result = secret_number / 32;
    // mix result into secret_number
    let secret_number = result ^ secret_number;
    // prune secret_number
    let secret_number = secret_number % 16777216;

    let result = secret_number * 2048;
    // mix result into secret_number
    let secret_number = result ^ secret_number;
    // prune secret_number
    secret_number % 16777216
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let codes = input.lines().map(tou64).collect();
        Puzzle {
            secret_numbers: codes,
        }
    }

    pub fn p1(&self) -> usize {
        let mut sum = 0;
        for &secret_number in &self.secret_numbers {
            let mut new_secret_number = secret_number;
            for _i in 0..2000 {
                new_secret_number = compute_nx(new_secret_number);
            }
            // println!("{}: {}", secret_number, new_secret_number);
            sum += new_secret_number as usize;
        }
        sum
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.p1()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> usize {
//     let puzzle = Puzzle::from_str(input);
//     puzzle.solve(25)
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day22: Keypad Conundrum");
    time_it(p1, "p1", "data/22_sample.txt");
    time_it(p1, "p1", "data/22_input.txt");
    // time_it(p2, "p2", "data/22_sample.txt");
    // time_it(p2, "p2", "data/22_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/22_sample.txt"), 37327623);
        // assert_eq!(run_it(p1, "data/22_input.txt"), 188398);
        // assert_eq!(run_it(p2, "data/22_sample.txt"), 154115708116294);
        // assert_eq!(run_it(p2, "data/22_input.txt"), 230049027535970);
    }
}
