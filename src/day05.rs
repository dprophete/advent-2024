use crate::utils::*;
use std::cmp::Ordering;

type Rule = (i32, i32);
type Rules = Vec<Rule>;
type Update = Vec<i32>;
type Updates = Vec<Update>;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_input(file_content: &str) -> (Rules, Updates) {
    let (first, second) = file_content.split_once("\n\n").unwrap();

    let rules: Rules = first
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (toi32(a), toi32(b)))
        .collect();

    let updates: Updates = second
        .lines()
        .map(|line| line.split(",").map(toi32).collect())
        .collect();

    (rules, updates)
}

fn are_pages_in_order(rules: &Rules, p1: i32, p2: i32) -> bool {
    !rules.contains(&(p2, p1))
}

fn is_update_valid(rules: &Rules, update: &Update) -> bool {
    update.windows(2).all(|window| {
        let [p1, p2] = &window else { unreachable!() };
        are_pages_in_order(rules, *p1, *p2)
    })
}

fn p1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    let mut sum = 0;
    for update in updates {
        if is_update_valid(&rules, &update) {
            sum += update[update.len() / 2];
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn order_pages(rules: &Rules, update: &mut Update) {
    update.sort_by(|p1, p2| {
        if are_pages_in_order(rules, *p1, *p2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}

fn p2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    let mut sum = 0;
    for mut update in updates {
        if !is_update_valid(&rules, &update) {
            order_pages(&rules, &mut update);
            sum += update[update.len() / 2];
        }
    }
    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    time_it(p1, "data/05_sample.txt");
    time_it(p1, "data/05_input.txt");
    time_it(p2, "data/05_sample.txt");
    time_it(p2, "data/05_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(run_it(p1, "data/05_sample.txt"), 143);
    }

    #[test]
    fn test_p2() {
        assert_eq!(run_it(p2, "data/05_sample.txt"), 123);
    }
}
