/// compaction
use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn pp_blocks(dm: &[Option<u32>]) {
    for &block in dm {
        match block {
            Some(file) => print!("{}", file),
            None => print!("."),
        }
    }
    println!();
}

fn diskmap_to_blocks(dm: &[u32]) -> Vec<Option<u32>> {
    let mut is_free_space = false;
    let mut block_nb = 0;
    let mut blocks: Vec<Option<u32>> = Vec::new();

    for &nb in dm {
        let block = if is_free_space { None } else { Some(block_nb) };
        for _ in 0..nb {
            blocks.push(block);
        }
        if !is_free_space {
            block_nb += 1;
        }
        is_free_space = !is_free_space;
    }
    blocks
}

fn compact_blocks_p1(blocks: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut compacted: Vec<Option<u32>> = vec![];

    let mut idx_empty = 0;
    let mut idx_file = blocks.len() - 1;

    while idx_file >= idx_empty {
        while blocks[idx_empty].is_some() && idx_file >= idx_empty {
            compacted.push(blocks[idx_empty]);
            idx_empty += 1;
        }
        while blocks[idx_file].is_none() && idx_file >= idx_empty {
            idx_file -= 1;
        }
        if idx_empty >= idx_file {
            break;
        }
        compacted.push(blocks[idx_file]);
        idx_empty += 1;
        idx_file -= 1;
    }
    compacted.to_vec()
}

fn checksum(blocks: &[Option<u32>]) -> usize {
    let mut sum = 0;
    for (idx, &o_file) in blocks.iter().enumerate() {
        if let Some(file) = o_file {
            sum += idx * (file as usize);
        }
    }
    sum
}

fn p1(input: &str) -> usize {
    let disk_map = input.chars().map(c_tou32).collect::<Vec<u32>>();
    let blocks = diskmap_to_blocks(&disk_map);
    // pp_blocks(&blocks);
    let compacted = compact_blocks_p1(&blocks);
    // pp_blocks(&compacted);
    checksum(&compacted)
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn find_empty_of_size(blocks: &[Option<u32>], size: usize) -> Option<usize> {
    let mut count = 0;
    for (i, &block) in blocks.iter().enumerate() {
        count = match block {
            Some(_) => 0,
            None => count + 1,
        };
        if count == size {
            return Some(i - size + 1);
        }
    }
    None
}

fn compact_blocks_p2(blocks: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut compacted: Vec<Option<u32>> = blocks.to_vec();

    let mut idx_file = blocks.len() - 1;
    let mut file = blocks[blocks.len() - 1].unwrap();

    while file > 0 {
        // find file
        while compacted[idx_file] != Some(file) {
            idx_file -= 1;
        }
        // size of file
        let mut size_of_file = 0;
        while compacted[idx_file] == Some(file) {
            size_of_file += 1;
            idx_file -= 1;
        }
        match find_empty_of_size(&compacted, size_of_file) {
            Some(idx_empty) if idx_empty <= idx_file => {
                for i in 0..size_of_file {
                    compacted[idx_file + size_of_file - i] = None;
                    compacted[idx_empty + i] = Some(file);
                }
            }
            _ => {}
        }
        file -= 1;
    }
    compacted.to_vec()
}

fn p2(input: &str) -> usize {
    let disk_map = input.chars().map(c_tou32).collect::<Vec<u32>>();
    let blocks = diskmap_to_blocks(&disk_map);
    // pp_blocks(&blocks);
    let compacted = compact_blocks_p2(&blocks);
    // pp_blocks(&compacted);
    checksum(&compacted)
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day09: compaction");
    time_it(p1, "p1", "data/09_sample.txt");
    time_it(p1, "p1", "data/09_input.txt");
    time_it(p2, "p2", "data/09_sample.txt");
    // time_it(p2, "data/09_input.txt"); // takes a few seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/09_sample.txt"), 1928);
        assert_eq!(run_it(p2, "data/09_sample.txt"), 2858);
    }
}
