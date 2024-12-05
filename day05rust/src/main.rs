#![allow(dead_code)]
#![feature(array_chunks)]

use std::{cmp::Ordering, fs};

type Rule = (i32, i32);
type Rules = Vec<Rule>;
type Update = Vec<i32>;
type Updates = Vec<Update>;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_file(file_content: &str) -> (Rules, Updates) {
    let (first, second) = file_content.split_once("\n\n").unwrap();

    let rules: Rules = first
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect();

    let updates: Updates = second
        .lines()
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
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

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let (rules, updates) = parse_file(&file_content);

    let mut sum = 0;
    for update in updates {
        if is_update_valid(&rules, &update) {
            sum += update[update.len() / 2];
        }
    }

    println!("p1 sum for {} -> {}", input, sum);
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

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");
    let (rules, updates) = parse_file(&file_content);

    let mut sum = 0;
    for mut update in updates {
        if !is_update_valid(&rules, &update) {
            order_pages(&rules, &mut update);
            sum += update[update.len() / 2];
        }
    }

    println!("p2 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

fn main() {
    p1("sample.txt");
    p1("input.txt");
    p2("sample.txt");
    p2("input.txt");
}
