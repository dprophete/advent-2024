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
        let racetrack = Matrix::from_str(input, identity).clone_without_border();
        let start = racetrack.find_first('S').unwrap();
        let end = racetrack.find_first('E').unwrap();

        Puzzle {
            racetrack,
            start,
            end,
        }
    }

    // pub fn pp_savings(&self, savings_for_cheat: &HashMap<(V2, V2), usize>) {
    //     let mut nb_cheats_for_saving = HashMap::new();
    //     for (_cheats, saving) in savings_for_cheat {
    //         let current = nb_cheats_for_saving.get(&saving).unwrap_or(&0);
    //         nb_cheats_for_saving.insert(saving, current + 1);
    //     }
    //
    //     for &saving in nb_cheats_for_saving.keys().sorted() {
    //         let nb_cheats = nb_cheats_for_saving.get(&saving).unwrap();
    //         println!("{} cheats saved {} picoseconds", nb_cheats, saving);
    //     }
    // }

    pub fn solve_p1(&self, threshold: i32) -> usize {
        let mut track = self.racetrack.clone();
        let mut pos = self.start;
        let mut cost = 0;
        let mut path = vec![];

        // basically a map of pos -> cost (ie index of pos on track)
        let mut cost_at_pos = HashMap::new();

        while pos != self.end {
            track.set(&pos, '-');
            cost += 1;
            cost_at_pos.insert(pos, cost);
            path.push(pos);

            let mut nx_pos = None;
            for nabe in track.neighbors(&pos) {
                if track.get(&nabe) == Some('.') || track.get(&nabe) == Some('E') {
                    nx_pos = Some(nabe);
                    break;
                }
            }
            pos = nx_pos.unwrap();
        }
        cost_at_pos.insert(pos, cost + 1);

        let mut shortcuts = HashMap::new();
        let mut track = self.racetrack.clone();
        track.set(&self.end, '.');
        for (mut cost, &pos) in path.iter().enumerate() {
            cost += 1;
            track.set(&pos, 'X');
            for cheat1 in track.neighbors(&pos) {
                if track.get(&cheat1) != Some('#') {
                    // cheat1 needs to be on a wall
                    continue;
                }
                for cheat2 in track.neighbors(&cheat1) {
                    if cheat2 == pos || track.get(&cheat2) != Some('.') {
                        // cheat2 needs to be back on the path
                        continue;
                    }
                    let &cost_at_cheat2 = cost_at_pos.get(&cheat2).unwrap();
                    let saving = cost_at_cheat2 - (cost as i32) - 2;
                    if saving > 0 && saving >= threshold {
                        // we have a shortcut !!!
                        let saving = saving as usize;
                        let &current = shortcuts.get(&(cheat1, cheat2)).unwrap_or(&0);
                        if saving > current {
                            shortcuts.insert((cheat1, cheat2), saving);
                        }
                    }
                }
            }
        }

        // self.pp_savings(&shortcuts);
        shortcuts.len()
    }

    pub fn solve_p2(&self, threshold: i32, cheat_length: i32) -> usize {
        let mut track = self.racetrack.clone();
        let mut pos = self.start;
        let mut cost = 0;
        let mut path = vec![];

        // basically a map of pos -> cost (ie index of pos on track)
        let mut cost_at_pos = HashMap::new();

        while pos != self.end {
            track.set(&pos, '-');
            cost += 1;
            cost_at_pos.insert(pos, cost);
            path.push(pos);

            let mut nx_pos = None;
            for nabe in track.neighbors(&pos) {
                if track.get(&nabe) == Some('.') || track.get(&nabe) == Some('E') {
                    nx_pos = Some(nabe);
                    break;
                }
            }
            pos = nx_pos.unwrap();
        }
        cost_at_pos.insert(pos, cost + 1);

        let mut shortcuts = HashMap::new();
        let mut track = self.racetrack.clone();
        track.set(&self.end, '.');
        for (mut cost, &pos) in path.iter().enumerate() {
            cost += 1;
            // track.set(&pos, 'X');
            let cheat1 = pos;
            for x in (pos.x - cheat_length)..=(pos.x + cheat_length) {
                // if x < 0 || x >= track.width {
                //     continue;
                // }
                for y in (pos.y - cheat_length)..=(pos.y + cheat_length) {
                    // if y < 0 || y >= track.height {
                    //     continue;
                    // }
                    if !track.is_in(&V2::new(x, y)) {
                        continue;
                    }
                    let dist = (pos.x - x).abs() + (pos.y - y).abs();
                    if dist == 0 || dist > cheat_length {
                        continue;
                    }

                    let cheat2 = V2::new(x, y);
                    if cheat2 == pos || track.get(&cheat2) != Some('.') {
                        // cheat2 needs to be back on the path
                        continue;
                    }
                    let &cost_at_cheat2 = cost_at_pos.get(&cheat2).unwrap();
                    let saving = cost_at_cheat2 - (cost as i32) - dist;
                    if saving > 0 && saving >= threshold {
                        // we have a shortcut !!!
                        let saving = saving as usize;
                        let &current = shortcuts.get(&(cheat1, cheat2)).unwrap_or(&0);
                        if saving > current {
                            shortcuts.insert((cheat1, cheat2), saving);
                        }
                    }
                }
            }
        }

        // self.pp_savings(&shortcuts);
        shortcuts.len()
    }
}

fn p1(input: &str, threshold: i32) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve_p1(threshold)
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str, threshold: i32) -> usize {
    let puzzle = Puzzle::from_str(input);
    puzzle.solve_p2(threshold, 20)
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day20: Race Condition");
    time_it(|input| p1(input, 0), "p1", "data/20_sample.txt");
    time_it(|input| p1(input, 100), "p1", "data/20_input.txt");
    time_it(|input| p2(input, 50), "p2", "data/20_sample.txt");
    time_it(|input| p2(input, 100), "p2", "data/20_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(|input| p1(input, 0), "data/20_sample.txt"), 44);
        assert_eq!(run_it(|input| p1(input, 100), "data/20_input.txt"), 1429);
        assert_eq!(run_it(|input| p2(input, 50), "data/20_sample.txt"), 285);
        assert_eq!(run_it(|input| p2(input, 50), "data/20_input.txt"), 988931);
    }
}
