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
            graph.entry(l).or_insert(vec![]).push(r);
        }

        let computers: HashSet<usize> = graph.keys().cloned().collect();
        Puzzle {
            connections,
            graph,
            computers,
        }
    }

    // fn dfs(&self, node: &str, visited: &mut Vec<String>) {
    //     visited.push(node.to_string());
    //     if let Some(neighbors) = self.graph.get(node) {
    //         for neighbor in neighbors {
    //             if !visited.contains(neighbor) {
    //                 self.dfs(neighbor, visited);
    //             }
    //         }
    //     }
    // }

    pub fn p1(&self) -> usize {
        let valid_comps = self
            .computers
            .iter()
            .filter(|&c| comp_id_to_comp_name(*c).starts_with("t"))
            .collect::<HashSet<_>>();

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
                        && (valid_comps.contains(&c1) || valid_comps.contains(&c2) || valid_comps.contains(&c3))
                    {
                        let mut triplet = [c1, c2, c3];
                        triplet.sort();
                        triplets.insert(triplet);
                    }
                }
            }
        }

        // for &c1 in self.computers.iter() {
        //     for &c2 in self.computers.iter() {
        //         if c1 == c2 {
        //             continue;
        //         }
        //         for &c3 in self.computers.iter() {
        //             if c1 == c3 || c2 == c3 {
        //                 continue;
        //             }
        //             if !valid_comps.contains(&c1) && !valid_comps.contains(&c2) && !valid_comps.contains(&c3) {
        //                 continue;
        //             }
        //             if self.connections.contains(&(c1, c2))
        //                 && self.connections.contains(&(c2, c3))
        //                 && self.connections.contains(&(c3, c1))
        //             {
        //                 let mut triplet = [c1, c2, c3];
        //                 triplet.sort();
        //                 triplets.insert(triplet);
        //             }
        //         }
        //     }
        // }

        triplets.len()
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);

    // println!("[DDA] day23:: {} -> {}", "ta", comp_name_to_comp_id("ta"));
    // println!("[DDA] day23:: {} -> {}", 24948, comp_id_to_comp_name(24948));
    // println!(
    //     "[DDA] day23:: #computers: {}, #conn {}",
    //     puzzle.computers.len(),
    //     puzzle.connections.len(),
    // );
    puzzle.p1()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> usize {
//     let puzzle = Puzzle::from_str(input);
//     puzzle.p2()
// }

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
