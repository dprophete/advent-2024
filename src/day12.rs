/// Garden Groups
use crate::utils::*;
use std::{collections::HashSet, convert::identity};

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn find_regions(matrix: &Matrix<char>) -> Vec<(char, HashSet<V2>)> {
    let mut matrix = matrix.clone();
    let mut regions = vec![];
    for j in 0..matrix.size {
        for i in 0..matrix.size {
            let pos = V2::new(i, j);
            match matrix.get(&pos) {
                Some('#') => {}
                Some(c) => {
                    // we are starting a new regions with this char
                    matrix.set(&pos, '#');
                    let mut region = HashSet::new();
                    region.insert(pos);
                    let mut to_explore = vec![pos];
                    while let Some(p1) = to_explore.pop() {
                        for n in matrix.neighbors(&p1) {
                            if matrix.get(&n) == Some(c) {
                                matrix.set(&n, '#');
                                region.insert(n);
                                to_explore.push(n);
                            }
                        }
                    }
                    regions.push((c, region));
                }
                _ => {}
            }
        }
    }
    regions
}

fn get_area_for_region(region: &HashSet<V2>) -> usize {
    region.len()
}

fn get_perimeter_for_region(region: &HashSet<V2>) -> usize {
    let mut sides = HashSet::new();
    for pos in region {
        for dir in [V2::UP, V2::DOWN, V2::LEFT, V2::RIGHT] {
            let side = pos.scale(2).add(&dir);
            if sides.contains(&side) {
                sides.remove(&side);
            } else {
                sides.insert(side);
            }
        }
    }

    sides.len()
}

fn p1(input: &str) -> usize {
    let matrix = Matrix::from_str(input, identity);
    let regions = find_regions(&matrix);
    let mut sum = 0;
    for (_c, region) in &regions {
        sum += get_area_for_region(region) * get_perimeter_for_region(region);
    }
    sum
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn get_nb_sides_for_region(region: &HashSet<V2>) -> usize {
    let mut sides = HashSet::new();
    for pos in region {
        for dir in [V2::UP, V2::DOWN, V2::LEFT, V2::RIGHT] {
            let side = pos.scale(2).add(&dir);
            if sides.contains(&side) {
                sides.remove(&side);
            } else {
                sides.insert(side);
            }
        }
    }

    sides.len()
}

fn p2(input: &str) -> usize {
    let matrix = Matrix::from_str(input, identity);
    let regions = find_regions(&matrix);
    let mut sum = 0;
    for (c, region) in &regions {
        println!(
            "[DDA] day12:: region {} -> {}",
            c,
            get_nb_sides_for_region(region)
        );
        sum += get_area_for_region(region) * get_nb_sides_for_region(region);
    }
    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day12: Garden Groups");
    // time_it(p1, "p1", "data/12_sample1a.txt");
    // time_it(p1, "p1", "data/12_sample1b.txt");
    // time_it(p1, "p1", "data/12_sample1c.txt");
    // time_it(p1, "p1", "data/12_input.txt");
    time_it(p2, "p2", "data/12_sample1a.txt");
    // time_it(p2, "p2", "data/12_input.txt"); // takes a few seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/12_sample1a.txt"), 140);
        assert_eq!(run_it(p1, "data/12_sample1b.txt"), 772);
        assert_eq!(run_it(p1, "data/12_sample1c.txt"), 1930);
        assert_eq!(run_it(p2, "data/12_sample1a.txt"), 80);
        assert_eq!(run_it(p2, "data/12_sample1b.txt"), 436);
        assert_eq!(run_it(p2, "data/12_sample1c.txt"), 1206);
        assert_eq!(run_it(p2, "data/12_sample2a.txt"), 236);
        assert_eq!(run_it(p2, "data/12_sample2b.txt"), 368);
    }
}
