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
                    // move boxes
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

impl Matrix<char> {
    fn put_box_at(&mut self, pos: V2) -> &mut Self {
        self.set(&pos, '[');
        self.set(&pos.add(&V2::RIGHT), ']');
        self
    }

    fn put_hori_boxes_at(&mut self, pos: V2, nb: i32) -> &mut Self {
        for i in 0..nb {
            self.put_box_at(pos.add(&V2::RIGHT.scale(2 * i)));
        }
        self
    }
}

fn can_move_boxes_in_dir(matrix: &mut Matrix<char>, box_to_check: V2, dir: V2) -> bool {
    let mut boxes_to_move_up = vec![];
    let mut boxes_to_check = vec![box_to_check];
    while let Some(box_to_check) = boxes_to_check.pop() {
        boxes_to_move_up.push(box_to_check);
        let up_left = box_to_check.add(&dir);
        let up_right = up_left.add(&V2::RIGHT);
        match (matrix.get(&up_left), matrix.get(&up_right)) {
            (Some('.'), Some('.')) => {
                // nothing above... we can move up
            }
            (Some('.'), Some('[')) => {
                // we have one box above, on the rigth
                boxes_to_check.push(up_right);
            }
            (Some(']'), Some('.')) => {
                // we have one box above, on the left
                boxes_to_check.push(up_left.add(&V2::LEFT));
            }
            (Some('['), Some(']')) => {
                // we have one box right above
                boxes_to_check.push(up_left);
            }
            (Some(']'), Some('[')) => {
                // we have two boxes above
                boxes_to_check.push(up_left.add(&V2::LEFT));
                boxes_to_check.push(up_right);
            }
            _ => {
                // we can't move up
                return false;
            }
        }
    }
    // move boxes up
    if dir == V2::UP {
        boxes_to_move_up.sort_by_key(|v| v.y);
    } else {
        boxes_to_move_up.sort_by_key(|v| -v.y);
    }
    for b in boxes_to_move_up {
        matrix.set(&b, '.');
        matrix.set(&b.add(&V2::RIGHT), '.');
        matrix.set(&b.add(&dir), '[');
        matrix.set(&b.add(&dir).add(&V2::RIGHT), ']');
    }
    true
}

fn p2(input: &str) -> i32 {
    let (map, dirs) = input.split_once("\n\n").unwrap();
    let dirs = dirs.lines().collect::<Vec<_>>().join("");
    let map2 = map
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let mut matrix = Matrix::from_str(&map2, identity);
    let mut robot = matrix.find_first('@').unwrap();
    matrix.set(&robot, '.');

    for c in dirs.chars() {
        // pp_with_robot(&mut matrix, robot);
        // println!("[DDA] day15:: trying to move {}", c);
        let dir = char_dir_to_v2(c);
        matrix.set(&robot, '.');
        let nx = robot.add(&dir);
        match matrix.get(&nx) {
            Some('.') => {
                robot = nx;
            }

            Some(']') => {
                match dir {
                    V2::LEFT => {
                        // count how many boxes we have
                        let mut nx_box = nx.add(&dir).add(&dir);
                        let mut nb_boxes = 1;
                        while matrix.get(&nx_box) == Some(']') {
                            nx_box = nx_box.add(&dir).add(&dir);
                            nb_boxes += 1;
                        }
                        // see if we can move them
                        if matrix.get(&nx_box) == Some('.') {
                            matrix.put_hori_boxes_at(nx_box, nb_boxes);
                            matrix.set(&robot, '.');
                            robot = nx;
                        }
                    }
                    V2::UP => {
                        if can_move_boxes_in_dir(&mut matrix, nx.add(&V2::LEFT), V2::UP) {
                            robot = nx;
                        }
                    }
                    V2::DOWN => {
                        if can_move_boxes_in_dir(&mut matrix, nx.add(&V2::LEFT), V2::DOWN) {
                            robot = nx;
                        }
                    }
                    _ => {}
                }
            }

            Some('[') => {
                match dir {
                    V2::RIGHT => {
                        // count how many boxes we have
                        let mut nx_box = nx.add(&dir).add(&dir);
                        let mut nb_boxes = 1;
                        while matrix.get(&nx_box) == Some('[') {
                            nx_box = nx_box.add(&dir).add(&dir);
                            nb_boxes += 1;
                        }
                        // see if we can move them
                        if matrix.get(&nx_box) == Some('.') {
                            matrix.put_hori_boxes_at(nx.add(&dir), nb_boxes);
                            robot = nx;
                        }
                    }
                    V2::UP => {
                        if can_move_boxes_in_dir(&mut matrix, nx, V2::UP) {
                            robot = nx;
                        }
                    }
                    V2::DOWN => {
                        if can_move_boxes_in_dir(&mut matrix, nx, V2::DOWN) {
                            robot = nx;
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
    // pp_with_robot(&mut matrix, robot);
    let boxes = matrix.find_all('[');
    boxes.iter().map(|v| v.x + v.y * 100).sum()
}

//--------------------------------------------------------------------------------
// main
//--------------------------------------------------------------------------------

pub fn run() {
    pp_day("day15: Warehouse Woes");
    time_it(p1, "p1", "data/15_sample_small.txt");
    time_it(p1, "p1", "data/15_sample.txt");
    time_it(p1, "p1", "data/15_input.txt");
    time_it(p2, "p2", "data/15_sample.txt");
    time_it(p2, "p2", "data/15_input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run_it(p1, "data/15_sample.txt"), 10092);
        assert_eq!(run_it(p1, "data/15_sample_small.txt"), 2028);
        assert_eq!(run_it(p2, "data/15_sample.txt"), 9021);
    }
}
