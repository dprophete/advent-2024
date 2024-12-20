use std::{collections::HashMap, convert::identity};

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
        // let mut potential_cheats = vec![];
        // let mut visited_steps = HashSet::new();
        // for &step in &initial_path {
        //     visited_steps.insert(step);
        //     if step == self.end {
        //         continue;
        //     }
        //     for cheat1 in self.racetrack.neighbors(&step) {
        //         if self.racetrack.get(&cheat1) != Some('#') {
        //             continue;
        //         }
        //         // we could cheat here
        //         for cheat2 in self.racetrack.neighbors(&cheat1) {
        //             if self.racetrack.get(&cheat2) != Some('#') {
        //                 if !potential_cheats.contains(&(cheat1, cheat2)) {
        //                     potential_cheats.push((cheat1, cheat2));
        //                 }
        //             }
        //         }
        //     }
        // }
        // println!(
        //     "[DDA] day20::nb potential_cheats {:?}",
        //     potential_cheats.len()
        // );

        // let's build a map with the cost at each step
        let mut cost_at_step = HashMap::new();
        for (i, &step) in initial_path.iter().enumerate() {
            cost_at_step.insert(step, i + 1);
        }

        // map: cheat -> saving
        let mut savings_for_cheat = HashMap::new();

        let mut visited = HashMap::new();
        for (pos, cost) in cost_at_step {
            visited.insert((pos, None), cost);
        }

        println!(
            "[DDA] day20::nb_steps_initial - threshold {:?}",
            nb_steps_initial - threshold
        );
        // we keep track of pos, steps, has cheated
        let mut to_explore = vec![(self.start, 0, None)];
        while let Some((pos, nb_steps, cheats)) = to_explore.pop() {
            if nb_steps > nb_steps_initial - threshold {
                continue;
            }
            if pos == self.end {
                match cheats {
                    Some((cheat1, cheat2)) => {
                        let savings = nb_steps_initial - nb_steps;
                        // println!(
                        //     "[DDA] day20:: cheating {:?} savings: {}",
                        //     (cheat1, cheat2),
                        //     savings
                        // );
                        let &current_savings =
                            savings_for_cheat.get(&(cheat1, cheat2)).unwrap_or(&0);
                        if savings > current_savings {
                            savings_for_cheat.insert((cheat1, cheat2), savings);
                        }
                    }
                    None => {
                        // println!("[DDA] day20:: no cheating, steps: {}", steps);
                        // great, we got the initial path...
                    }
                }
                continue;
            }
            if self.racetrack.get(&pos) == Some('#') {
                match cheats {
                    Some((_cheat1, _cheat2)) => {
                        // too bad... we already cheated and hit another wall here...
                    }
                    None => {
                        // no cheats yet, let's start exploring the potential cheat paths here
                        let cheat1 = pos;
                        for cheat2 in self.racetrack.neighbors(&cheat1) {
                            if self.racetrack.get(&cheat2) != Some('#') {
                                // we want to make sure we don't end up in a wall here
                                to_explore.push((cheat2, nb_steps + 1, Some((cheat1, cheat2))));
                            }
                        }
                    }
                }
                // do we need to go through the rest of this ?
                continue;
            }

            if let Some(&nb_steps_at_pos_no_cheat) = visited.get(&(pos, None)) {
                // we need to do at least better than the no-cheat version
                match cheats {
                    Some(_) => {
                        // if we have already cheated, then we need to do better than the no-cheat
                        // version + the threshold
                        if nb_steps_at_pos_no_cheat < nb_steps + threshold {
                            continue;
                        }
                    }
                    None => {
                        // we haven't cheated yet... we are matching the no-cheat version
                    }
                }
            }
            if let Some(&nb_steps_at_pos) = visited.get(&(pos, cheats)) {
                if nb_steps_at_pos <= nb_steps {
                    continue;
                }
            }
            visited.insert((pos, cheats), nb_steps);

            for nx_pos in self.racetrack.neighbors(&pos) {
                to_explore.push((nx_pos, nb_steps + 1, cheats));
            }
        }

        let mut total_savings: usize = 0;
        let mut nb_cheats_for_saving = HashMap::new();
        for (_cheats, saving) in savings_for_cheat {
            let current = nb_cheats_for_saving.get(&saving).unwrap_or(&0);
            nb_cheats_for_saving.insert(saving, current + 1);
        }

        for &saving in nb_cheats_for_saving.keys().sorted() {
            if saving < threshold {
                continue;
            }
            let nb_cheats = nb_cheats_for_saving.get(&saving).unwrap();
            total_savings += nb_cheats;
            // println!("{} cheats saved {} picoseconds", nb_cheats, saving);
        }

        // println!("[DDA] day20::savings_for_cheat {:?}", savings_for_cheat);
        // for  in savings_for_cheat {
        //     if saving < threshold {
        //         continue;
        //     }
        //     let nb_cheats = nb_cheats_for_saving.get(&saving).unwrap();
        //     total_savings += nb_cheats;
        //     // println!("{} cheats saved {} picoseconds", nb_cheats, saving);
        // }

        // let mut min_steps_with_cheat = nb_steps_initial;
        // let mut nb_cheats_for_saving = HashMap::new();
        // for (cheat1, cheat2) in potential_cheats {
        //     let mut new_racetrack = self.racetrack.clone();
        //     new_racetrack.set(&cheat1, '1');
        //     new_racetrack.set(&cheat2, '2');
        //
        //     // println!("\n{}", new_racetrack);
        //     // try to solve it
        //     let mut min_steps_with_cheat = nb_steps_initial - threshold;
        //     let mut visited = cost_at_step.clone();
        //     // we keep track of pos, steps, has cheated
        //     let mut to_explore = vec![(self.start, 0, false)];
        //     while let Some((pos, steps, has_cheated)) = to_explore.pop() {
        //         if steps >= min_steps_with_cheat {
        //             continue;
        //         }
        //         if pos == self.end {
        //             if steps < min_steps_with_cheat {
        //                 min_steps_with_cheat = steps;
        //             }
        //             continue;
        //         }
        //         if new_racetrack.get(&pos) == Some('#') {
        //             continue;
        //         }
        //         if new_racetrack.get(&pos) == Some('2') && !has_cheated {
        //             continue;
        //         }
        //         if let Some(&steps_at_pos) = visited.get(&pos) {
        //             if steps_at_pos <= steps {
        //                 continue;
        //             }
        //         }
        //         visited.insert(pos, steps);
        //
        //         if pos == cheat1 {
        //             to_explore.push((cheat2, steps + 1, true));
        //         } else {
        //             for nx_pos in new_racetrack.neighbors(&pos) {
        //                 to_explore.push((nx_pos, steps + 1, false));
        //             }
        //         }
        //     }
        //     if min_steps_with_cheat < nb_steps_initial {
        //         let savings = nb_steps_initial - min_steps_with_cheat;
        //         let current = nb_cheats_for_saving.get(&savings).unwrap_or(&0);
        //         nb_cheats_for_saving.insert(savings, current + 1);
        //     }
        // }
        //
        // for &saving in nb_cheats_for_saving.keys().sorted() {
        //     if saving < threshold {
        //         continue;
        //     }
        //     let nb_cheats = nb_cheats_for_saving.get(&saving).unwrap();
        //     total_savings += nb_cheats;
        //     // println!("{} cheats saved {} picoseconds", nb_cheats, saving);
        // }

        total_savings
    }
}

// 14 cheats saved 2 picoseconds
// 14 cheats saved 4 picoseconds
// 2 cheats saved 6 picoseconds
// 4 cheats saved 8 picoseconds
// 2 cheats saved 10 picoseconds
// 3 cheats saved 12 picoseconds
// 1 cheats saved 20 picoseconds
// 1 cheats saved 36 picoseconds
// 1 cheats saved 38 picoseconds
// 1 cheats saved 40 picoseconds
// 1 cheats saved 64 picoseconds
// [5ms] p1 : data/20_sample.txt -> 44

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
        // assert_eq!(run_it(|input| p1(input, 100), "data/20_sample.txt"), 1429);
        //         assert_eq!(run_it(p1, "data/20_input.txt"), 242);
        //         assert_eq!(run_it(p2, "data/20_sample.txt"), 16);
        //         assert_eq!(run_it(p2, "data/20_input.txt"), 595975512785325);
    }
}
