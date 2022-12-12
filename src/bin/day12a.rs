use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/day12.txt");

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heights = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (match c {
                        'S' => {
                            start = (x, y);
                            'a'
                        }
                        'E' => {
                            end = (x, y);
                            'z'
                        }
                        _ => c,
                    }) as u8
                        - b'a'
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_x = heights[0].len() - 1;
    let max_y = heights.len() - 1;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if !visited.insert(pos) {
            continue;
        } else if pos == end {
            println!("Steps: {}", steps);
            return;
        } else {
            let max_height = heights[pos.1][pos.0] + 1;
            if pos.0 > 0 && heights[pos.1][pos.0 - 1] <= max_height {
                queue.push_back(((pos.0 - 1, pos.1), steps + 1));
            }
            if pos.1 > 0 && heights[pos.1 - 1][pos.0] <= max_height {
                queue.push_back(((pos.0, pos.1 - 1), steps + 1));
            }
            if pos.0 < max_x && heights[pos.1][pos.0 + 1] <= max_height {
                queue.push_back(((pos.0 + 1, pos.1), steps + 1));
            }
            if pos.1 < max_y && heights[pos.1 + 1][pos.0] <= max_height {
                queue.push_back(((pos.0, pos.1 + 1), steps + 1));
            }
        }
    }
    println!("No path!");
}
