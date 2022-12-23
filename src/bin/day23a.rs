use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day23.txt");

fn print_elves(elves: &HashSet<(i32, i32)>) {
    let (minx, maxx) = elves
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = elves
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    for y in miny..=maxy {
        let mut s = String::new();
        for x in minx..=maxx {
            s.push(if elves.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("{}", s);
    }
    println!();
}

fn main() {
    let mut elves: HashSet<(i32, i32)> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let mut initial_dir = 0;
    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    let dir_offsets = [0, 4, 6, 2];
    let mut moves = HashMap::<_, Vec<_>>::new();
    for _ in 0..10 {
        print_elves(&elves);
        for &(x, y) in &elves {
            let occupied: Vec<_> = offsets
                .iter()
                .map(|&(dx, dy)| elves.contains(&(x + dx, y + dy)))
                .collect();
            let (k, v) = if occupied.contains(&true) {
                let mut res = None;
                for dir_idx in 0..4 {
                    let dir = (initial_dir + dir_idx) % 4;
                    if !(0..3).any(|i| occupied[(dir_offsets[dir] + i) % 8]) {
                        let (dx, dy) = offsets[dir_offsets[dir] + 1];
                        res = Some(((x + dx, y + dy), (x, y)));
                        break;
                    }
                }
                res.unwrap_or(((x, y), (x, y)))
            } else {
                ((x, y), (x, y))
            };
            moves.entry(k).or_default().push(v);
        }
        elves.clear();
        for (k, vs) in moves.drain() {
            if vs.len() == 1 {
                elves.insert(k);
            } else {
                elves.extend(vs);
            }
        }

        initial_dir = (initial_dir + 1) % 4;
    }

    let (minx, maxx) = elves
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = elves
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    println!(
        "{}",
        (maxx + 1 - minx) * (maxy + 1 - miny) - elves.len() as i32
    );
}
