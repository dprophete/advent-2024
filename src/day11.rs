/// blinking stones
use crate::utils::*;
use std::collections::HashMap;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

// add count to the value in the hashmap
fn inc_value_by_count(map: &mut HashMap<u64, usize>, key: u64, count: usize) {
    map.insert(key, map.get(&key).unwrap_or(&0) + count);
}

fn change_stones(stones_map: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut res = HashMap::new();
    for (&stone, &count) in stones_map {
        if stone == 0 {
            inc_value_by_count(&mut res, 1, count);
        } else {
            let nb_digits = stone.ilog10() as usize + 1;
            if nb_digits % 2 == 0 {
                let p = 10_u64.pow(nb_digits as u32 / 2);
                let right = stone % p;
                let left = stone / p;

                inc_value_by_count(&mut res, left, count);
                inc_value_by_count(&mut res, right, count);
            } else {
                inc_value_by_count(&mut res, stone * 2024, count);
            }
        }
    }
    res
}

fn iter_n_times(stones: Vec<u64>, n: i32) -> usize {
    let mut stones_map = stones
        .iter()
        .map(|&stone| (stone, 1))
        .collect::<HashMap<_, _>>();

    for _ in 0..n {
        stones_map = change_stones(&stones_map);
    }
    stones_map.values().sum::<usize>()
}

fn p1(input: &str) -> usize {
    let stones = input.split_whitespace().map(tou64).collect::<Vec<_>>();
    iter_n_times(stones, 25)
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> usize {
    let stones = input.split_whitespace().map(tou64).collect::<Vec<_>>();
    iter_n_times(stones, 75)
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day11: blinking stones");
    time_it(p1, "data/11_sample.txt");
    time_it(p1, "data/11_input.txt");
    time_it(p2, "data/11_sample.txt");
    time_it(p2, "data/11_input.txt"); // takes a few seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/11_sample.txt"), 55312);
        assert_eq!(run_it(p2, "data/11_sample.txt"), 65601038650482);
    }
}
