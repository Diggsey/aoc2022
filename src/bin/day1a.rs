const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let max_calories: u64 = INPUT
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|l| l.parse::<u64>().unwrap())
                .sum()
        })
        .max()
        .unwrap();
    println!("{}", max_calories);
}
