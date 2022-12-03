const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let score: u64 = INPUT
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(" ").unwrap();
            let win_score = match (a, b) {
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
                _ => panic!("{} {}", a, b),
            };
            let shape_score = match b {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };
            win_score + shape_score
        })
        .sum();
    println!("{}", score);
}
