use crate::utils::*;
use std::{collections::HashSet, convert::identity};

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn find_regions(matrix: &Matrix<char>) -> Vec<(char, HashSet<V2>)> {
    let mut matrix = matrix.clone();
    let mut regions = vec![];
    for j in 0..matrix.height {
        for i in 0..matrix.width {
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

    // we scale the region by 2 and put the fences in the 'middle'
    let scaled_region = region
        .iter()
        .map(|pos| pos.scale(2))
        .collect::<HashSet<_>>();
    for pos in scaled_region {
        for dir in [V2::UP, V2::DOWN, V2::LEFT, V2::RIGHT] {
            let side = pos.add(&dir);
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

// true if a fence is on top of the region
fn is_top_fence(pos: &V2, scaled_region: &HashSet<V2>) -> bool {
    scaled_region.contains(&pos.add(&V2::DOWN))
}

// true if a fence is on the left of the region
fn is_left_fence(pos: &V2, scaled_region: &HashSet<V2>) -> bool {
    scaled_region.contains(&pos.add(&V2::RIGHT))
}

fn get_nb_sides_for_region(region: &HashSet<V2>) -> usize {
    let mut sides = HashSet::new();

    // we scale the region by 2 and put the fences in the 'middle'
    let scaled_region = region
        .iter()
        .map(|pos| pos.scale(2))
        .collect::<HashSet<_>>();
    for pos in scaled_region.clone() {
        for dir in [V2::UP, V2::DOWN, V2::LEFT, V2::RIGHT] {
            let side = pos.add(&dir);
            if sides.contains(&side) {
                sides.remove(&side);
            } else {
                sides.insert(side);
            }
        }
    }

    // same as p1, but we make sure the fences are on the same side
    let mut discarded = HashSet::new();
    for side in sides.clone() {
        if side.x % 2 == 0 {
            // horizontal
            let right = side.add(&V2::RIGHT.scale(2));
            let is_top_fence1 = bool_to_u32(is_top_fence(&side, &scaled_region));
            let is_top_fence2 = bool_to_u32(is_top_fence(&right, &scaled_region));
            if sides.contains(&right) && (is_top_fence1 == is_top_fence2) {
                discarded.insert(right);
            }
        } else {
            // vertical
            let down = side.add(&V2::DOWN.scale(2));
            let is_left_fence1 = bool_to_u32(is_left_fence(&side, &scaled_region));
            let is_left_fence2 = bool_to_u32(is_left_fence(&down, &scaled_region));
            if sides.contains(&down) && (is_left_fence1 == is_left_fence2) {
                discarded.insert(down);
            }
        }
    }
    sides.len() - discarded.len()
}

fn p2(input: &str) -> usize {
    let matrix = Matrix::from_str(input, identity);
    let regions = find_regions(&matrix);
    let mut sum = 0;
    for (_c, region) in &regions {
        sum += get_area_for_region(region) * get_nb_sides_for_region(region);
    }
    sum
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day12: Garden Groups");
    time_it(p1, "p1", "data/12_input.txt");
    time_it(p2, "p2", "data/12_input.txt");
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
