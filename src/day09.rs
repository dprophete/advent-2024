use crate::utils::*;

// use itertools::Itertools;
// use std::collections::{HashMap, HashSet};

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

// fn pp_disk_map(dm: &[u32]) {
//     let mut is_free_space = false;
//     let mut block_nb = 0;
//     for &nb in dm {
//         if is_free_space {
//             print!("{}", ".".repeat(nb as usize));
//         } else {
//             print!("{}", format!("{}", block_nb).repeat(nb as usize));
//             block_nb += 1;
//         }
//         is_free_space = !is_free_space;
//     }
//     println!();
// }

fn pp_blocks(dm: &[u32]) {
    for &nb in dm {
        if nb == 0 {
            print!(".");
        } else {
            print!("{}", nb - 1);
        }
    }
    println!();
}

fn to_blocks(dm: &[u32]) -> Vec<u32> {
    let mut is_free_space = false;
    let mut block_nb = 0;
    let mut blocks: Vec<u32> = Vec::new();

    for &nb in dm {
        let x = if is_free_space { 0 } else { block_nb + 1 };
        for _ in 0..nb {
            blocks.push(x);
        }
        if !is_free_space {
            block_nb += 1;
        }
        is_free_space = !is_free_space;
    }
    blocks
}

fn compact_blocks_p1(blocks: &[u32]) -> Vec<u32> {
    let mut compacted: Vec<u32> = vec![];

    let mut idx_empty = 0;
    let mut idx_block = blocks.len() - 1;

    while idx_block >= idx_empty {
        while blocks[idx_empty] != 0 && idx_block >= idx_empty {
            compacted.push(blocks[idx_empty]);
            idx_empty += 1;
        }
        while blocks[idx_block] == 0 && idx_block >= idx_empty {
            idx_block -= 1;
        }
        if idx_empty >= idx_block {
            break;
        }
        compacted.push(blocks[idx_block]);
        idx_empty += 1;
        idx_block -= 1;
    }
    compacted.to_vec()
}

fn checksum(blocks: &[u32]) -> u32 {
    let mut sum = 0;
    for (idx, &nb) in blocks.iter().enumerate() {
        sum += (idx as u32) * (nb - 1);
    }
    sum
}

fn p1(input: &str) -> u32 {
    let disk_map = input.chars().map(c_tou32).collect::<Vec<u32>>();
    let blocks = to_blocks(&disk_map);
    println!("[DDA] day09::blocks {:?}", blocks);
    pp_blocks(&blocks);
    let compacted = compact_blocks_p1(&blocks);
    pp_blocks(&compacted);
    checksum(&compacted)
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn compact_blocks_p2(blocks: &[u32]) -> Vec<u32> {
    let mut compacted: Vec<u32> = vec![];

    let mut idx_empty = 0;
    let mut idx_block = blocks.len() - 1;

    while idx_block >= idx_empty {
        while blocks[idx_empty] != 0 && idx_block >= idx_empty {
            compacted.push(blocks[idx_empty]);
            idx_empty += 1;
        }
        while blocks[idx_block] == 0 && idx_block >= idx_empty {
            idx_block -= 1;
        }
        if idx_empty >= idx_block {
            break;
        }
        compacted.push(blocks[idx_block]);
        idx_empty += 1;
        idx_block -= 1;
    }
    compacted.to_vec()
}

fn p2(input: &str) -> u32 {
    let disk_map = input.chars().map(c_tou32).collect::<Vec<u32>>();
    let blocks = to_blocks(&disk_map);
    pp_blocks(&blocks);
    let compacted = compact_blocks_p2(&blocks);
    pp_blocks(&compacted);
    checksum(&compacted)
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    time_it(p1, "data/09_sample.txt");
    // time_it(p1, "data/09_input.txt");
    // time_it(p2, "data/09_sample.txt");
    // time_it(p2, "data/09_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/09_sample.txt"), 1928);
        assert_eq!(run_it(p2, "data/09_sample.txt"), 2958);
    }
}
