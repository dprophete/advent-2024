use std::{collections::HashMap, convert::identity};

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

        // we keep track of pos, steps, has cheated
        let mut to_explore = vec![(self.start, 0, None)];
        while let Some((pos, nb_steps, cheats)) = to_explore.pop() {
            // we are already beyond the threshold ?
            match cheats {
                Some(_) => {
                    if nb_steps_initial < nb_steps + threshold {
                        continue;
                    }
                    if let Some(&nb_steps_at_pos_no_cheat) = visited.get(&(pos, None)) {
                        // if we have already cheated, then we need to do better than the no-cheat
                        // version + the threshold
                        if nb_steps_at_pos_no_cheat < nb_steps + threshold {
                            continue;
                        }
                    }
                }
                None => {
                    if nb_steps > nb_steps_initial {
                        continue;
                    }
                }
            }

            // we reached the end
            if pos == self.end {
                match cheats {
                    Some((cheat1, cheat2)) => {
                        let savings = nb_steps_initial - nb_steps;
                        let &current_savings =
                            savings_for_cheat.get(&(cheat1, cheat2)).unwrap_or(&0);
                        if savings > current_savings {
                            savings_for_cheat.insert((cheat1, cheat2), savings);
                        }
                    }
                    None => {
                        // great, we got the initial path...
                    }
                }
                continue;
            }

            // we hit a wall
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
                continue;
            }

            // are we beating our own score ?
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

        // let mut nb_cheats_for_saving = HashMap::new();
        // for (_cheats, saving) in savings_for_cheat {
        //     let current = nb_cheats_for_saving.get(&saving).unwrap_or(&0);
        //     nb_cheats_for_saving.insert(saving, current + 1);
        // }
        //
        // for &saving in nb_cheats_for_saving.keys().sorted() {
        //     if saving < threshold {
        //         continue;
        //     }
        //     let nb_cheats = nb_cheats_for_saving.get(&saving).unwrap();
        //     // println!("{} cheats saved {} picoseconds", nb_cheats, saving);
        // }

        savings_for_cheat.len()
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
