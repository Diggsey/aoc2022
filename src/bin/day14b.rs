use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day14.txt");

fn main() {
    let mut solid_cells = HashSet::new();
    for (a, b) in INPUT.lines().flat_map(|line| {
        line.split(" -> ")
            .map(|coord| {
                let (x, y) = coord.split_once(',').unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .tuple_windows()
    }) {
        let dist = a.0.abs_diff(b.0).max(a.1.abs_diff(b.1)) as i32;
        for i in 0..=dist {
            let pos = (
                a.0 + (b.0 - a.0).signum() * i,
                a.1 + (b.1 - a.1).signum() * i,
            );
            solid_cells.insert(pos);
        }
    }
    let max_y = solid_cells.iter().map(|pos| pos.1).max().unwrap() + 1;

    'outer: for grain in 0.. {
        let mut pos = (500, 0);
        loop {
            if pos.1 < max_y {
                pos.1 += 1;
                if !solid_cells.contains(&pos) {
                    continue;
                }
                pos.0 -= 1;
                if !solid_cells.contains(&pos) {
                    continue;
                }
                pos.0 += 2;
                if !solid_cells.contains(&pos) {
                    continue;
                }
                pos.0 -= 1;
                pos.1 -= 1;
            }
            if solid_cells.insert(pos) {
                continue 'outer;
            } else {
                break;
            }
        }
        println!("{}", grain);
        break;
    }
}
