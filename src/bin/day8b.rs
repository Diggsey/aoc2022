const INPUT: &str = include_str!("../../inputs/day8.txt");

fn view_dist(h: u32, m: usize, mut it: impl Iterator<Item = u32>) -> usize {
    it.position(|elem| elem >= h).map(|d| d + 1).unwrap_or(m)
}

fn main() {
    let digits: Vec<u32> = INPUT.chars().filter_map(|c| c.to_digit(10)).collect();
    let width = INPUT.find("\n").unwrap();
    let height = digits.len() / width;
    let digits = &digits;

    let res = (0..height)
        .flat_map(move |y| {
            (0..width).map(move |x| {
                let h = digits[y * width + x];
                let a = view_dist(h, x, (0..x).rev().map(|nx| digits[y * width + nx]));
                let b = view_dist(
                    h,
                    width - x - 1,
                    (x + 1..width).map(|nx| digits[y * width + nx]),
                );
                let c = view_dist(h, y, (0..y).rev().map(|ny| digits[ny * width + x]));
                let d = view_dist(
                    h,
                    height - y - 1,
                    (y + 1..height).map(|ny| digits[ny * width + x]),
                );
                a * b * c * d
            })
        })
        .max()
        .unwrap();
    println!("{}", res);
}
