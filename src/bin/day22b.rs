use std::{collections::HashSet, io::stdin, mem};

const INPUT: &str = include_str!("../../inputs/day22.txt");

const FACE_SIZE: usize = 50;

fn find_new_pos(
    mut pos: (usize, usize),
    mut dir: usize,
    map: &[Vec<char>],
) -> Option<((usize, usize), usize)> {
    match dir {
        0 => {
            pos.0 += 1;
            if pos.0 >= map[0].len() {
                pos.0 = 0;
            }
        }
        1 => {
            pos.1 += 1;
            if pos.1 >= map.len() {
                pos.1 = 0;
            }
        }
        2 => {
            if pos.0 == 0 {
                pos.0 = map[0].len();
            }
            pos.0 -= 1;
        }
        3 => {
            if pos.1 == 0 {
                pos.1 = map.len();
            }
            pos.1 -= 1;
        }
        _ => unreachable!(),
    }
    loop {
        match map[pos.1].get(pos.0).copied().unwrap_or(' ') {
            '.' => break Some((pos, dir)),
            '#' => break None,
            ' ' => {
                let mut facex = pos.0 / FACE_SIZE;
                let mut facey = pos.1 / FACE_SIZE;
                pos.0 = pos.0 % FACE_SIZE;
                pos.1 = pos.1 % FACE_SIZE;
                match (facex, facey, dir) {
                    (0, 0, 0) => {
                        facex = 1;
                        facey = 2;
                        dir = 2;
                        pos.0 = FACE_SIZE - pos.0 - 1;
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (0, 0, 1) => {
                        facex = 2;
                    }
                    (0, 0, 2) => {
                        facey = 2;
                        dir = 0;
                        pos.0 = FACE_SIZE - pos.0 - 1;
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (0, 1, 2) => {
                        facey = 2;
                        dir = 1;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (0, 1, 3) => {
                        facex = 1;
                        dir = 0;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.0 = FACE_SIZE - pos.0 - 1;
                    }
                    (2, 1, 0) => {
                        facey = 0;
                        dir = 3;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (2, 1, 1) => {
                        facex = 1;
                        dir = 2;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.0 = FACE_SIZE - pos.0 - 1;
                    }
                    (2, 2, 0) => {
                        facey = 0;
                        dir = 2;
                        pos.0 = FACE_SIZE - pos.0 - 1;
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (2, 2, 2) => {
                        facex = 1;
                        facey = 0;
                        dir = 0;
                        pos.0 = FACE_SIZE - pos.0 - 1;
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (1, 3, 0) => {
                        facey = 2;
                        dir = 3;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (1, 3, 1) => {
                        facex = 0;
                        dir = 2;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.0 = FACE_SIZE - pos.0 - 1;
                    }
                    (1, 3, 3) => {
                        facex = 0;
                        dir = 0;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.0 = FACE_SIZE - pos.0 - 1;
                    }
                    (2, 3, 2) => {
                        facex = 1;
                        facey = 0;
                        dir = 1;
                        mem::swap(&mut pos.0, &mut pos.1);
                        pos.1 = FACE_SIZE - pos.1 - 1;
                    }
                    (2, 3, 3) => {
                        facex = 0;
                    }
                    _ => unreachable!(),
                }
                pos.0 += facex * FACE_SIZE;
                pos.1 += facey * FACE_SIZE;
            }
            _ => unreachable!(),
        }
    }
}

#[allow(unused)]
fn print_map(
    pos: (usize, usize),
    positions: &HashSet<(usize, usize)>,
    dir: usize,
    map: &[Vec<char>],
) {
    let mut res = String::new();
    for y in pos.1.saturating_sub(60)..map.len().min(pos.1 + 60) {
        for x in 0..map[y].len() {
            if positions.contains(&(x, y)) {
                res += "\u{001b}[31m";
                res.push(match dir {
                    0 => '>',
                    1 => 'v',
                    2 => '<',
                    3 => '^',
                    _ => unreachable!(),
                });
                res += "\u{001b}[0m";
            } else {
                res.push(map[y][x]);
            };
        }
        res.push('\n');
    }
    println!("{}", res);
    stdin().read_line(&mut res).unwrap();
}

fn main() {
    let (map_str, dir_str) = INPUT.split_once("\n\n").unwrap();
    let map: Vec<Vec<_>> = map_str.lines().map(|line| line.chars().collect()).collect();
    let mut pos = (map[0].iter().position(|c| *c != ' ').unwrap(), 0);
    let mut dir = 0;
    for direction in dir_str.trim().split_inclusive(['L', 'R']) {
        let (step_str, turn) = if let Some(step_str) = direction.strip_suffix('L') {
            (step_str, 3)
        } else if let Some(step_str) = direction.strip_suffix('R') {
            (step_str, 1)
        } else {
            (direction, 0)
        };
        let steps = step_str.parse().unwrap();
        let mut positions = HashSet::new();
        positions.insert(pos);
        for _ in 0..steps {
            if let Some((new_pos, new_dir)) = find_new_pos(pos, dir, &map) {
                pos = new_pos;
                dir = new_dir;
            } else {
                break;
            }
            positions.insert(pos);
        }
        //print_map(pos, &positions, dir, &map);
        dir = (dir + turn) % 4;
    }
    println!("{:?}", 1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir);
}
