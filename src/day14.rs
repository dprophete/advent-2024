use regex::Regex;

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Robot {
    p: V2,
    v: V2,
}

impl Robot {
    pub fn from_str(s: &str) -> Robot {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let (_, [px, py, vx, vy]) = re.captures(s).unwrap().extract();
        Robot {
            p: V2::new(toi32(px), toi32(py)),
            v: V2::new(toi32(vx), toi32(vy)),
        }
    }

    pub fn step(&self, area: &V2) -> Robot {
        Robot {
            p: self.p.add(&self.v).modulo(area),
            v: self.v,
        }
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from_str).collect()
}

fn step_robots(robots: &[Robot], area: &V2) -> Vec<Robot> {
    robots.iter().map(|robot| robot.step(area)).collect()
}

fn p1(area: V2, input: &str) -> i32 {
    let mut robots = parse_robots(input);
    for _ in 0..100 {
        robots = step_robots(&robots, &area);
    }

    // compute quadrants
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bot_left = 0;
    let mut bot_right = 0;

    let center = V2::new((area.x - 1) / 2, (area.y - 1) / 2);
    for robot in &robots {
        if robot.p.x == center.x || robot.p.y == center.y {
            continue;
        }
        match (robot.p.x < center.x, robot.p.y < center.y) {
            (true, true) => top_left += 1,
            (false, true) => top_right += 1,
            (true, false) => bot_left += 1,
            (false, false) => bot_right += 1,
        }
    }

    top_left * top_right * bot_left * bot_right
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn contains_line(robots: &[Robot], area: &V2) -> bool {
    let mut grid = vec![vec!['.'; area.x as usize]; area.y as usize];
    for robot in robots {
        if grid[robot.p.y as usize][robot.p.x as usize] == '.' {
            grid[robot.p.y as usize][robot.p.x as usize] = 'X';
        }
    }
    for row in grid {
        let str = row.iter().collect::<String>();
        // println!("{}", str);
        if str.contains("XXXXXXXXXXXXXXXXXXX") {
            return true;
        }
    }
    false
}

fn p2(area: V2, input: &str) -> i32 {
    // part 2 was tricky...
    //   this is what I did:
    //   basically, I ran it 10000 times, outputted everthing in a file,
    //   and searched for a row of XXXXXXXXXXXXX
    //     cargo r | grep -B 100 XXXXXXXXXXXXXXXX | less

    // now programatically
    let mut robots = parse_robots(input);
    for i in 0..10000 {
        if contains_line(&robots, &area) {
            return i;
        }
        robots = step_robots(&robots, &area);
    }
    -1
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day14: Restroom Redoubt");
    time_it(|input| p1(V2::new(11, 7), input), "p1", "data/14_sample.txt");
    time_it(|input| p1(V2::new(101, 103), input), "p1", "data/14_input.txt");
    time_it(|input| p2(V2::new(101, 103), input), "p1", "data/14_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(|input| p1(V2::new(11, 7), input), "data/14_sample.txt"), 12);
        assert_eq!(run_it(|input| p2(V2::new(101, 103), input), "data/14_input.txt"), 6516);
    }
}
