use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use itertools::Itertools;

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Puzzle {
    racetrack: Matrix<char>,
    start: V2,
    end: V2,
}

impl Puzzle {
    pub fn from_str(input: &str) -> Puzzle {
        let mut racetrack = Matrix::from_str(input, identity);
        racetrack.has_border = true;
        let start = racetrack.find_first('S').unwrap();
        let end = racetrack.find_first('E').unwrap();

        Puzzle {
            racetrack,
            start,
            end,
        }
    }

    pub fn solve(&self, threshold: usize) -> usize {
        // first, find the path
        let mut track = self.racetrack.clone();
        let mut pos = self.start;
        let mut initial_path = vec![];
        while pos != self.end {
            track.set(&pos, 'X');
            initial_path.push(pos);

            let mut nx_pos = None;
            for nabe in track.neighbors(&pos) {
                if track.get(&nabe) == Some('.') || track.get(&nabe) == Some('E') {
                    nx_pos = Some(nabe);
                    break;
                }
            }
            pos = nx_pos.unwrap();
        }
        let nb_steps_initial = initial_path.len();
        println!("[DDA] day20:: nb_steps_initial: {}", nb_steps_initial);

        // now go along the path and figure out where we would want to cheat
        let mut potential_cheats = vec![];
        let mut visited_steps = HashSet::new();
        for &step in &initial_path {
            visited_steps.insert(step);
            if step == self.end {
                continue;
            }
            for cheat1 in self.racetrack.neighbors(&step) {
                if self.racetrack.get(&cheat1) != Some('#') {
                    continue;
                }
                // we could cheat here
                for cheat2 in self.racetrack.neighbors(&cheat1) {
                    if self.racetrack.get(&cheat2) != Some('#') {
                        if !potential_cheats.contains(&(cheat1, cheat2)) {
                            potential_cheats.push((cheat1, cheat2));
                        }
                    }
                }
            }
        }
        println!(
            "[DDA] day20::nb potential_cheats {:?}",
            potential_cheats.len()
        );

        // let's build a map with the cost at each step
        let mut cost_at_step = HashMap::new();
        for (i, &step) in initial_path.iter().enumerate() {
            cost_at_step.insert(step, i + 1);
        }

        // now solve for each potential cheat
        let mut min_steps_for_cheats = HashMap::new();
        for (cheat1, cheat2) in potential_cheats {
            let mut new_racetrack = self.racetrack.clone();
            new_racetrack.set(&cheat1, '1');
            new_racetrack.set(&cheat2, '2');

            // println!("\n{}", new_racetrack);
            // try to solve it
            let mut min_steps_with_cheat = nb_steps_initial;
            let mut visited = cost_at_step.clone();
            // we keep track of pos, steps, has cheated
            let mut to_explore = vec![(self.start, 0, false)];
            while let Some((pos, steps, has_cheated)) = to_explore.pop() {
                if pos == self.end {
                    if steps < min_steps_with_cheat {
                        min_steps_with_cheat = steps;
                    }
                    continue;
                }
                if new_racetrack.get(&pos) == Some('#') {
                    continue;
                }
                if new_racetrack.get(&pos) == Some('2') && !has_cheated {
                    continue;
                }
                if let Some(&steps_at_pos) = visited.get(&pos) {
                    if steps_at_pos <= steps {
                        continue;
                    }
                }
                visited.insert(pos, steps);

                if pos == cheat1 {
                    to_explore.push((cheat2, steps + 1, true));
                } else {
                    for nx_pos in new_racetrack.neighbors(&pos) {
                        to_explore.push((nx_pos, steps + 1, false));
                    }
                }
            }
            if min_steps_with_cheat < nb_steps_initial {
                let savings = nb_steps_initial - min_steps_with_cheat;
                let current = min_steps_for_cheats.get(&savings).unwrap_or(&0);
                min_steps_for_cheats.insert(savings, current + 1);
            }
        }

        let mut total_savings: usize = 0;
        for &saving in min_steps_for_cheats.keys().sorted() {
            if saving < threshold {
                continue;
            }
            let nb_cheats = min_steps_for_cheats.get(&saving).unwrap();
            total_savings += nb_cheats;
            // println!("{} cheats saved {} picoseconds", nb_cheats, saving);
        }
        total_savings
    }
}

fn p1(input: &str, threshold: usize) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve(threshold)
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

// fn p2(input: &str) -> u64 {
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day20: Race Condition");
    time_it(|input| p1(input, 0), "p1", "data/20_sample.txt");
    time_it(|input| p1(input, 100), "p1", "data/20_input.txt");
    // time_it(p2, "p2", "data/20_sample.txt");
    // time_it(p2, "p2", "data/20_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(|input| p1(input, 0), "data/20_sample.txt"), 44);
        //         assert_eq!(run_it(p1, "data/20_input.txt"), 242);
        //         assert_eq!(run_it(p2, "data/20_sample.txt"), 16);
        //         assert_eq!(run_it(p2, "data/20_input.txt"), 595975512785325);
    }
}
