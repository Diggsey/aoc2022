use std::cmp::Reverse;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let calories: u64 = INPUT
        .split("\n\n")
        .map(|elf_inventory| {
            Reverse(
                elf_inventory
                    .lines()
                    .map(|l| l.parse::<u64>().unwrap())
                    .sum::<u64>(),
            )
        })
        .k_smallest(3)
        .map(|x| x.0)
        .sum();
    println!("{}", calories);
}
