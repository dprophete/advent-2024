use std::fs;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_line(line: &str) -> (i32, i32) {
    let vals = line
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [l, r, ..] = vals.as_slice() else {
        panic!("invalid input")
    };
    (*l, *r)
}

fn p1(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = file_content.lines().map(parse_line).unzip();
    left.sort();
    right.sort();

    let pairs: Vec<(i32, i32)> = left.into_iter().zip(right).collect();
    let sum = pairs.iter().map(|(l, r)| (r - l).abs()).sum::<i32>();

    println!("p1 sum for {} -> {}", input, sum);
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) {
    let file_content = fs::read_to_string(input).expect("cannot read sample file");

    let (left, right): (Vec<i32>, Vec<i32>) = file_content.lines().map(parse_line).unzip();
    let mut sum = 0;
    for l in left.iter() {
        // not super optimized, but it works
        let nb_matches: i32 = right.iter().filter(|r| *r == l).count() as i32;
        sum += nb_matches * l;
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
