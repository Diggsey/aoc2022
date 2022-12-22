const INPUT: &str = include_str!("../../inputs/day22.txt");

fn find_new_pos(mut pos: (usize, usize), dir: usize, map: &[Vec<char>]) -> Option<(usize, usize)> {
    loop {
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
        match map[pos.1].get(pos.0).copied().unwrap_or(' ') {
            '.' => break Some(pos),
            '#' => break None,
            ' ' => continue,
            _ => unreachable!(),
        }
    }
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
        for _ in 0..steps {
            if let Some(new_pos) = find_new_pos(pos, dir, &map) {
                pos = new_pos;
            } else {
                break;
            }
        }
        dir = (dir + turn) % 4;
    }
    println!("{:?}", 1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir);
}
