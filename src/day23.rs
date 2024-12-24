use std::collections::{HashMap, HashSet};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    connections: HashSet<(usize, usize)>, // list of connections between computers
    graph: HashMap<usize, Vec<usize>>,    // for each computer, list of connected computers
    computers: HashSet<usize>,            // name of each computer
}

fn comp_name_to_comp_id(comp_name: &str) -> usize {
    let chars = comp_name.chars().collect::<Vec<_>>();
    let c0 = chars[0];
    let c1 = chars[1];
    (c0 as usize) + (c1 as usize) * 256
}

fn comp_id_to_comp_name(comp_id: usize) -> String {
    let c0 = char::from_u32((comp_id / 256) as u32).unwrap();
    let c1 = char::from_u32((comp_id % 256) as u32).unwrap();
    format!("{}{}", c1, c0)
}

fn is_starting_with_t(comp_id: usize) -> bool {
    let c1 = char::from_u32((comp_id % 256) as u32).unwrap();
    c1 == 't'
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        // connection: map (comp_id, comp_id)
        let connections: HashSet<(usize, usize)> = input
            .lines()
            .flat_map(|line| {
                let (l_str, r_str) = line.split_once("-").unwrap();
                let l = comp_name_to_comp_id(l_str);
                let r = comp_name_to_comp_id(r_str);
                [(l, r), (r, l)]
            })
            .collect();

        // prepare graph
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(l, r) in connections.iter() {
            graph.entry(l).or_default().push(r);
        }

        let computers: HashSet<usize> = graph.keys().cloned().collect();
        Puzzle {
            connections,
            graph,
            computers,
        }
    }

    pub fn p1(&self) -> usize {
        let mut triplets = HashSet::new();
        for (&c1, connections) in &self.graph {
            if connections.len() < 2 {
                continue;
            }
            for &c2 in connections.iter() {
                for &c3 in connections.iter() {
                    if c2 == c3 {
                        continue;
                    }
                    // we already have c1 <-> c2
                    // we already have c1 <-> c3
                    if self.connections.contains(&(c2, c3))
                        && (is_starting_with_t(c1) || is_starting_with_t(c2) || is_starting_with_t(c3))
                    {
                        let mut triplet = [c1, c2, c3];
                        triplet.sort();
                        triplets.insert(triplet);
                    }
                }
            }
        }
        triplets.len()
    }

    pub fn p2(&self) -> usize {
        // let mut triplets = HashSet::new();
        // for (&c1, connections) in &self.graph {
        //     if connections.len() < 2 {
        //         continue;
        //     }
        //     for &c2 in connections.iter() {
        //         for &c3 in connections.iter() {
        //             if c2 == c3 {
        //                 continue;
        //             }
        //             // we already have c1 <-> c2
        //             // we already have c1 <-> c3
        //             if self.connections.contains(&(c2, c3))
        //                 && (is_starting_with_t(c1) || is_starting_with_t(c2) || is_starting_with_t(c3))
        //             {
        //                 let mut triplet = [c1, c2, c3];
        //                 triplet.sort();
        //                 triplets.insert(triplet);
        //             }
        //         }
        //     }
        // }
        // triplets.len()
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
    pp_day("day23: LAN Party");
    time_it(p1, "p1", "data/23_sample.txt");
    time_it(p1, "p1", "data/23_input.txt");
    // time_it(p2, "p2", "data/23_sample.txt");
    // time_it(p2, "p2", "data/23_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/23_sample.txt"), 7);
        assert_eq!(run_it(p1, "data/23_input.txt"), 1284);
        // assert_eq!(run_it(p2, "data/23_sample.txt"), 23);
        // assert_eq!(run_it(p2, "data/23_input.txt"), 1612);
    }
}
