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

fn p1(area: V2, input: &str) -> i32 {
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

fn display_robots(robots: &[Robot], area: &V2) {
    let mut grid = vec![vec![0; area.x as usize]; area.y as usize];
    for robot in robots {
        grid[robot.p.y as usize][robot.p.x as usize] += 1;
    }
    for row in grid {
        for cell in row {
            if cell == 0 {
                print!(".");
            } else {
                print!("X");
            }
        }
        println!();
    }
}

fn p2(area: V2, input: &str) -> i32 {
    // part 2 is tricky...
    // basically, I ran it 10000 times, outputted everthing in a file,
    // and searched for a row of XXXXXXXXXXXXX
    //
    //   cargo r | grep -B 100 XXXXXXXXXXXXXXXX | less
    let mut robots = parse_robots(input);
    for i in 0..10000 {
        println!("\n----------------------- {}", i);
        display_robots(&robots, &area);
        robots = move_robots(&robots, &area);
    }
    10
}

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
    // time_it(
    //     |input| p2(V2::new(101, 103), input),
    //     "p1",
    //     "data/14_input.txt",
    // );
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
    }
}
