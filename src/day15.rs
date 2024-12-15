use std::convert::identity;

use crate::utils::*;

//--------------------------------------------------------------------------------
// p1
//--------------------------------------------------------------------------------

fn char_dir_to_v2(c: char) -> V2 {
    match c {
        '^' => V2::UP,
        'v' => V2::DOWN,
        '<' => V2::LEFT,
        '>' => V2::RIGHT,
        _ => panic!("invalid char dir"),
    }
}

fn pp_with_robot(matrix: &mut Matrix<char>, robot: V2) {
    matrix.set(&robot, '@');
    println!("{}", matrix);
    matrix.set(&robot, '.');
}

fn p1(input: &str) -> i32 {
    let (map, dirs) = input.split_once("\n\n").unwrap();
    let dirs = dirs.lines().collect::<Vec<_>>().join("");
    let mut matrix = Matrix::from_str(map, identity);
    let mut robot = matrix.find_first('@').unwrap();
    matrix.set(&robot, '.');

    for c in dirs.chars() {
        let dir = char_dir_to_v2(c);
        let nx = robot.add(&dir);
        match matrix.get(&nx) {
            Some('.') => robot = nx,
            Some('O') => {
                // count how many boxes we have
                let mut nx_box = nx;
                while matrix.get(&nx_box) == Some('O') {
                    nx_box = nx_box.add(&dir);
                }
                // see if we can move them
                if matrix.get(&nx_box) == Some('.') {
                    matrix.set(&nx_box, 'O');
                    matrix.set(&nx, '.');
                    robot = nx;
                }
            }
            _ => {}
        }
    }
    // pp_with_robot(&mut matrix, robot);
    let boxes = matrix.find_all('O');
    boxes.iter().map(|v| v.x + v.y * 100).sum()
}

//--------------------------------------------------------------------------------
// p2
//--------------------------------------------------------------------------------

fn p2(input: &str) -> i32 {
    let (map, dirs) = input.split_once("\n\n").unwrap();
    let dirs = dirs.lines().collect::<Vec<_>>().join("");
    let map2 = map
        .chars()
        .flat_map(|c| match c {
            '#' => vec!['#', '#'],
            'O' => vec!['[', ']'],
            '.' => vec!['.', '.'],
            '@' => vec!['@', '.'],
            _ => vec![c],
        })
        .collect::<String>();

    let mut matrix = Matrix::from_str(&map2, identity);
    let mut robot = matrix.find_first('@').unwrap();
    pp_with_robot(&mut matrix, robot);
    // matrix.set(&robot, '.');
    //
    // for c in dirs.chars() {
    //     let dir = char_dir_to_v2(c);
    //     let nx = robot.add(&dir);
    //     match matrix.get(&nx) {
    //         Some('.') => robot = nx,
    //         Some('O') => {
    //             // count how many boxes we have
    //             let mut nx_box = nx;
    //             while matrix.get(&nx_box) == Some('O') {
    //                 nx_box = nx_box.add(&dir);
    //             }
    //             // see if we can move them
    //             if matrix.get(&nx_box) == Some('.') {
    //                 matrix.set(&nx_box, 'O');
    //                 matrix.set(&nx, '.');
    //                 robot = nx;
    //             }
    //         }
    //         _ => {}
    //     }
    // }
    // // pp_with_robot(&mut matrix, robot);
    // let boxes = matrix.find_all('O');
    // boxes.iter().map(|v| v.x + v.y * 100).sum()
    10
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day15: Warehouse Woes");
    // time_it(p1, "p1", "data/15_sample_small.txt");
    // time_it(p1, "p1", "data/15_sample.txt");
    // time_it(p1, "p2", "data/15_input.txt");
    time_it(p2, "p2", "data/15_sample.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/15_sample.txt"), 10092);
        assert_eq!(run_it(p1, "data/15_sample_small.txt"), 2028);
    }
}
