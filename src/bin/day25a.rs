const INPUT: &str = include_str!("../../inputs/day25.txt");

fn convert_from_snafu(input: &str) -> i64 {
    input
        .chars()
        .rev()
        .zip(0..)
        .map(|(c, p)| {
            (match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            }) * 5i64.pow(p)
        })
        .sum()
}

fn convert_to_snafu(mut input: i64) -> String {
    let mut chars = Vec::new();
    while input != 0 {
        let (c, v) = match input.rem_euclid(5) {
            0 => ('0', 0),
            1 => ('1', 1),
            2 => ('2', 2),
            3 => ('=', -2),
            4 => ('-', -1),
            _ => unreachable!(),
        };
        input -= v;
        input = input.div_euclid(5);
        chars.push(c);
    }
    if chars.is_empty() {
        chars.push('0');
    }
    chars.into_iter().rev().collect()
}

fn main() {
    let res = INPUT.lines().map(convert_from_snafu).sum();
    let res2 = convert_to_snafu(res);
    println!("{}: {}: {}", res, res2, convert_from_snafu(&res2));
}
