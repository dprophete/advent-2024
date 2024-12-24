use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    connections: HashSet<(usize, usize)>, // list of connections between computers
    graph: HashMap<usize, Vec<usize>>,    // for each computer, list of connected computers
}

fn comp_name_to_comp_id(comp_name: &str) -> usize {
    let chars = comp_name.chars().collect::<Vec<_>>();
    let c0 = chars[0];
    let c1 = chars[1];
    (c1 as usize) + (c0 as usize) * 256
}

fn comp_id_to_comp_name(comp_id: usize) -> String {
    let c0 = char::from_u32((comp_id / 256) as u32).unwrap();
    let c1 = char::from_u32((comp_id % 256) as u32).unwrap();
    format!("{}{}", c0, c1)
}

fn comp_ids_to_comp_names(comp_ids: &[usize]) -> Vec<String> {
    comp_ids.iter().map(|&c| comp_id_to_comp_name(c)).collect()
}

fn is_starting_with_t(comp_id: usize) -> bool {
    let c0 = char::from_u32((comp_id / 256) as u32).unwrap();
    c0 == 't'
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

        Puzzle { connections, graph }
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

    pub fn pw_for_clusters_of_size_n(&self, n: usize) -> Option<String> {
        for (&c1, connections_to_c1) in &self.graph {
            if connections_to_c1.len() < n {
                continue;
            }
            for combs in connections_to_c1.iter().combinations(n) {
                // combs is a set of n computers connectred to c1

                // then let's check that they all have at least n connections
                let all_with_n_conns = combs.iter().all(|&&c| self.graph.get(&c).unwrap().len() >= n);
                if !all_with_n_conns {
                    continue;
                }

                // now check if they are all connected to each other
                let all_connected = combs
                    .iter()
                    .map(|&&c| c)
                    .combinations(2)
                    .all(|pair| self.connections.contains(&(pair[0], pair[1])));
                if !all_connected {
                    continue;
                }

                let mut comps_in_cluster: Vec<usize> = combs.iter().map(|&&c| c).collect();
                comps_in_cluster.push(c1);
                comps_in_cluster.sort();
                let pw = comps_in_cluster
                    .into_iter()
                    .map(comp_id_to_comp_name)
                    .collect::<Vec<_>>()
                    .join(",");
                return Some(pw);
            }
        }
        None
    }

    pub fn p2(&self) -> String {
        let max_connections = self.graph.values().map(|v| v.len()).max().unwrap();

        for i in (2..=max_connections).rev() {
            if let Some(pw) = self.pw_for_clusters_of_size_n(i) {
                return pw;
            }
        }
        String::new()
    }
}

fn p1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.p1()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> String {
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
    time_it(p2, "p2", "data/23_sample.txt");
    time_it(p2, "p2", "data/23_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/23_sample.txt"), 7);
        assert_eq!(run_it(p1, "data/23_input.txt"), 1284);
        assert_eq!(run_it(p2, "data/23_sample.txt"), "co,de,ka,ta");
        assert_eq!(
            run_it(p2, "data/23_input.txt"),
            "bv,cm,dk,em,gs,jv,ml,oy,qj,ri,uo,xk,yw"
        );
    }
}
