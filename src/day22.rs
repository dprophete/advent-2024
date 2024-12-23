use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    secret_numbers: Vec<i64>,
}

fn compute_nx(secret_number: i64) -> i64 {
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

fn find_sequence(a: i8, b: i8, c: i8, d: i8, changes: &Vec<i8>) -> Option<usize> {
    for i in 0..changes.len() - 4 {
        if changes[i] == a && changes[i + 1] == b && changes[i + 2] == c && changes[i + 3] == d {
            return Some(i);
        }
    }
    None
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let codes = input.lines().map(toi64).collect();
        Puzzle { secret_numbers: codes }
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

    pub fn p2(&self) -> usize {
        // let mut sum = 0;
        let mut changes_for_all_sellers = vec![];
        let mut ones_for_all_sellers = vec![];
        for &secret_number in &self.secret_numbers {
            let mut new_secret_number = secret_number;
            let mut ones_for_seller: Vec<i8> = vec![];
            let mut changes_for_seller: Vec<i8> = vec![];
            let mut current_ones = (new_secret_number % 10) as i8;
            for i in 0..2000 {
                new_secret_number = compute_nx(new_secret_number);
                let new_current_one = (new_secret_number % 10) as i8;
                ones_for_seller.push(new_current_one);
                changes_for_seller.push(new_current_one - current_ones);
                current_ones = new_current_one
            }
            match find_sequence(-2, 1, -1, 3, &changes_for_seller) {
                Some(i) => {
                    println!(
                        "[DDA] day22:: for {}, found at idx {} , value: {}",
                        secret_number,
                        i,
                        &ones_for_seller[i + 3]
                    );
                }
                None => {
                    println!("[DDA] day22:: no sequence found for {}", secret_number);
                }
            }
            changes_for_all_sellers.push(changes_for_seller);
            ones_for_all_sellers.push(ones_for_seller);
        }
        10
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.p1()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.p2()
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day22: Keypad Conundrum");
    // time_it(p1, "p1", "data/22_sample.txt");
    // time_it(p1, "p1", "data/22_input.txt");
    time_it(p2, "p2", "data/22_sample2.txt");
    // time_it(p2, "p2", "data/22_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/22_sample.txt"), 37327623);
        assert_eq!(run_it(p1, "data/22_input.txt"), 13584398738);
        assert_eq!(run_it(p2, "data/22_sample2.txt"), 23);
        // assert_eq!(run_it(p2, "data/22_input.txt"), 230049027535970);
    }
}
