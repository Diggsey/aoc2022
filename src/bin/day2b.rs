const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let score: u64 = INPUT
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(" ").unwrap();
            let shape_score = match (a, b) {
                ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
                ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
                _ => panic!("{} {}", a, b),
            };
            let win_score = match b {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => unreachable!(),
            };
            win_score + shape_score
        })
        .sum();
    println!("{}", score);
}
