use crate::utils::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Robot {
    p: V2,
    v: V2,
}

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn parse_dirs(line: &str) -> V2 {
    let (_, dirs) = line.split_once("=").unwrap();
    let (x_dir, y_dir) = dirs.split_once(",").unwrap();
    V2::new(toi32(x_dir), toi32(y_dir))
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" ").unwrap();
            Robot {
                p: parse_dirs(lhs),
                v: parse_dirs(rhs),
            }
        })
        .collect()
}

fn move_robot(robot: &Robot, area: &V2) -> Robot {
    Robot {
        p: robot.p.add(&robot.v).modulo(area),
        v: robot.v,
    }
}

fn move_robots(robots: &[Robot], area: &V2) -> Vec<Robot> {
    robots.iter().map(|robot| move_robot(robot, area)).collect()
}

fn p1(area: V2, input: &str) -> i64 {
    let mut robots = parse_robots(input);
    for _ in 0..100 {
        robots = move_robots(&robots, &area);
    }

    // compute quadrants
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bot_left = 0;
    let mut bot_right = 0;

    let center = V2::new((area.x - 1) / 2, (area.y - 1) / 2);
    for robot in robots {
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

// fn p2(input: &str) -> i64 {
//     let mut sum = 0;
//     for machine in parse_machines(input) {
//         sum += compute_cost(&machine, 10000000000000_i64, false);
//     }
//     sum
// }

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day14: Restroom Redoubt");
    time_it(
        |input| p1(V2::new(11, 7), input),
        "p1",
        "data/14_sample.txt",
    );
    time_it(
        |input| p1(V2::new(101, 103), input),
        "p1",
        "data/14_input.txt",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            run_it(|input| p1(V2::new(11, 7), input), "data/14_sample.txt"),
            12
        );
        // assert_eq!(run_it(p2, "data/14_sample.txt"), 875318608908);
    }
}
