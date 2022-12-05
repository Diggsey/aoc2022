use scan_fmt::scan_fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day5.txt");

#[derive(Default, Debug)]
struct Layout {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Layout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self::default();
        for line in s.lines().rev().skip(1) {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if i >= res.stacks.len() {
                    res.stacks.push(Vec::new());
                }
                if c != ' ' {
                    res.stacks[i].push(c);
                }
            }
        }
        Ok(res)
    }
}

impl Layout {
    fn move_one(&mut self, a: usize, b: usize) {
        let c = self.stacks[a].pop().unwrap();
        self.stacks[b].push(c);
    }
    fn move_n(&mut self, a: usize, b: usize, n: usize) {
        for _ in 0..n {
            self.move_one(a, b);
        }
    }
}

fn main() {
    let (initial_layout_str, moves_str) = INPUT.split_once("\n\n").unwrap();
    let mut layout: Layout = initial_layout_str.parse().unwrap();

    for move_str in moves_str.lines() {
        let (n, a, b) = scan_fmt!(move_str, "move {} from {} to {}", usize, usize, usize).unwrap();
        layout.move_n(a - 1, b - 1, n);
    }

    for stack in layout.stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
