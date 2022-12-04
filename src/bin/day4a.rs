use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/day4.txt");

fn to_range(s: &str) -> RangeInclusive<u32> {
    let (a, b) = s.split_once("-").unwrap();
    a.parse().unwrap()..=b.parse().unwrap()
}

fn is_subrange(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    b.start() >= a.start() && b.end() <= a.end()
}

fn main() {
    let count = INPUT
        .lines()
        .filter(|s| {
            let (a, b) = s.split_once(",").unwrap();
            let ar = to_range(a);
            let br = to_range(b);
            is_subrange(&ar, &br) || is_subrange(&br, &ar)
        })
        .count();
    println!("{}", count);
}
