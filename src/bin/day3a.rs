use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let score: u64 = INPUT
        .lines()
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            let set_a: HashSet<_> = a.chars().collect();
            let set_b: HashSet<_> = b.chars().collect();
            (match *set_a.intersection(&set_b).next().unwrap() as u8 {
                c @ b'a'..=b'z' => 1 + c - b'a',
                c @ b'A'..=b'Z' => 27 + c - b'A',
                _ => unreachable!(),
            }) as u64
        })
        .sum();
    println!("{}", score);
}
