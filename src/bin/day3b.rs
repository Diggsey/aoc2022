use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let score: u64 = INPUT
        .lines()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .tuples()
        .map(|(a, b, c)| {
            let d = &(&a & &b) & &c;
            (match d.into_iter().next().unwrap() as u8 {
                ch @ b'a'..=b'z' => 1 + ch - b'a',
                ch @ b'A'..=b'Z' => 27 + ch - b'A',
                _ => unreachable!(),
            }) as u64
        })
        .sum();
    println!("{}", score);
}
