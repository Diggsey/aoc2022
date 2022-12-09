use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day9.txt");

const N_TAILS: usize = 9;

fn main() {
    let mut head = (0i32, 0i32);
    let mut tails = [(0i32, 0i32); N_TAILS];
    let mut visited = HashSet::new();
    visited.insert(tails[N_TAILS - 1]);
    for line in INPUT.lines() {
        let (dir, dist_str) = line.split_once(" ").unwrap();
        let dist: usize = dist_str.parse().unwrap();
        for _ in 0..dist {
            head = match dir {
                "U" => (head.0, head.1 - 1),
                "D" => (head.0, head.1 + 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                _ => unreachable!(),
            };
            tails
                .iter_mut()
                .scan(head, |head, tail| {
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        tail.0 += (head.0 - tail.0).signum();
                        tail.1 += (head.1 - tail.1).signum();
                    }
                    *head = *tail;
                    Some(())
                })
                .count();
            visited.insert(tails[N_TAILS - 1]);
        }
    }
    println!("{}", visited.len());
}
