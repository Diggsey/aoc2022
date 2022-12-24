use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Clear,
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct QueueItem {
    f: u32,
    pos: (i32, i32, i32),
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f).reverse()
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueItem {}
impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

fn path_find(plane: &[Vec<Cell>], time: i32, start: (i32, i32), goal: (i32, i32)) -> i32 {
    let (w, h) = (plane[0].len() as i32, plane.len() as i32);
    let mut visited = HashSet::new();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(QueueItem {
        f: goal.0.abs_diff(start.0) + goal.1.abs_diff(start.1) + time as u32,
        pos: (start.0, start.1, time),
    });
    while let Some(item) = priority_queue.pop() {
        if !visited.insert(item.pos) {
            continue;
        }
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1), (0, 0)] {
            let new_pos = (item.pos.0 + dx, item.pos.1 + dy, item.pos.2 + 1);
            if new_pos.0 == goal.0 && new_pos.1 == goal.1 {
                return new_pos.2;
            }

            // Check bounds
            if new_pos.0 != start.0 || new_pos.1 != start.1 {
                if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= w || new_pos.1 >= h {
                    continue;
                }
                // Check for blizzard
                if plane[new_pos.1 as usize][((new_pos.0 + new_pos.2).rem_euclid(w)) as usize]
                    == Cell::West
                    || plane[new_pos.1 as usize][((new_pos.0 - new_pos.2).rem_euclid(w)) as usize]
                        == Cell::East
                    || plane[(new_pos.1 + new_pos.2).rem_euclid(h) as usize][new_pos.0 as usize]
                        == Cell::North
                    || plane[(new_pos.1 - new_pos.2).rem_euclid(h) as usize][new_pos.0 as usize]
                        == Cell::South
                {
                    continue;
                }
            }
            priority_queue.push(QueueItem {
                f: goal.0.abs_diff(new_pos.0) + goal.1.abs_diff(new_pos.1) + new_pos.2 as u32,
                pos: new_pos,
            });
        }
    }
    panic!("No path")
}

fn main() {
    let plane: Vec<Vec<Cell>> = INPUT
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with("##"))
        .map(|line| {
            line[1..line.len() - 1]
                .chars()
                .map(|c| match c {
                    '.' => Cell::Clear,
                    '^' => Cell::North,
                    '>' => Cell::East,
                    'v' => Cell::South,
                    '<' => Cell::West,
                    _ => panic!("Unknown cell: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (w, h) = (plane[0].len() as i32, plane.len() as i32);
    let goal = (w - 1, h);
    let start = (0, -1);
    let mut time = 0;
    time = path_find(&plane, time, start, goal);
    time = path_find(&plane, time, goal, start);
    time = path_find(&plane, time, start, goal);
    println!("{}", time);
}
