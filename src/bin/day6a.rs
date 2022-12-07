use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    const N: usize = 4;
    let index = INPUT
        .as_bytes()
        .windows(N)
        .position(|arr| arr.into_iter().all_unique())
        .unwrap()
        + N;
    println!("{}", index);
}
