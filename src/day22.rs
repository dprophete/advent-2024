use std::collections::{HashMap, HashSet};

use crate::utils::*;

// a sequence of 4 consecutive changes
type Seq = (i8, i8, i8, i8);

const SEQ_NOOP: i8 = -100; // anything outside of [-9, 9] is good...

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
        let mut seqs_for_all_sellers = vec![];
        let mut distinct_seqs = HashSet::new();

        for &secret_number in &self.secret_numbers {
            let mut new_secret_number = secret_number;
            let mut seqs_for_seller = HashMap::new();

            let mut current_ones = (new_secret_number % 10) as i8;

            let mut seq: Seq = (SEQ_NOOP, SEQ_NOOP, SEQ_NOOP, SEQ_NOOP);

            // we are going to generate 2000 changes
            for i in 0..2000 {
                new_secret_number = compute_nx(new_secret_number);

                let new_current_one = (new_secret_number % 10) as i8;
                let new_change = new_current_one - current_ones;
                current_ones = new_current_one;

                seq = (seq.1, seq.2, seq.3, new_change);
                if i >= 3 && !seqs_for_seller.contains_key(&seq) {
                    distinct_seqs.insert(seq);
                    seqs_for_seller.insert(seq, new_current_one);
                }
            }

            seqs_for_all_sellers.push(seqs_for_seller);
        }

        let mut max_nb_bananas = 0;
        for seq_to_test in distinct_seqs {
            let mut nb_bananas_for_seq = 0;
            for seqs_for_seller in &seqs_for_all_sellers {
                if let Some(&bananas) = seqs_for_seller.get(&seq_to_test) {
                    nb_bananas_for_seq += bananas as usize;
                }
            }
            if nb_bananas_for_seq > max_nb_bananas {
                max_nb_bananas = nb_bananas_for_seq;
            }
        }

        max_nb_bananas
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
    time_it(p1, "p1", "data/22_sample.txt");
    time_it(p1, "p1", "data/22_input.txt");
    time_it(p2, "p2", "data/22_sample2.txt");
    time_it(p2, "p2", "data/22_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/22_sample.txt"), 37327623);
        assert_eq!(run_it(p1, "data/22_input.txt"), 13584398738);
        assert_eq!(run_it(p2, "data/22_sample2.txt"), 23);
        assert_eq!(run_it(p2, "data/22_input.txt"), 1612);
    }
}
