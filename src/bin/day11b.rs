use std::{cmp::Reverse, collections::VecDeque, str::FromStr};

use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day11.txt");

#[derive(Debug)]
enum Expr {
    Old,
    Constant(u64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn parse_term(s: &mut &[&str]) -> Self {
        let (&term, new_s) = s.split_first().unwrap();
        *s = new_s;
        if term == "old" {
            Expr::Old
        } else {
            Expr::Constant(term.parse().unwrap())
        }
    }
    fn parse_mul(s: &mut &[&str]) -> Self {
        let mut expr = Self::parse_term(s);
        while let Some(new_s) = s.strip_prefix(&["*"]) {
            *s = new_s;
            expr = Expr::Mul(Box::new(expr), Box::new(Self::parse_term(s)));
        }
        expr
    }
    fn parse_add(s: &mut &[&str]) -> Self {
        let mut expr = Self::parse_mul(s);
        while let Some(new_s) = s.strip_prefix(&["+"]) {
            *s = new_s;
            expr = Expr::Add(Box::new(expr), Box::new(Self::parse_mul(s)));
        }
        expr
    }
    fn eval(&self, old: u64) -> u64 {
        match self {
            Expr::Old => old,
            Expr::Constant(v) => *v,
            Expr::Add(a, b) => a.eval(old) + b.eval(old),
            Expr::Mul(a, b) => a.eval(old) * b.eval(old),
        }
    }
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        Ok(Self::parse_add(&mut parts.as_slice()))
    }
}

#[derive(Debug)]
struct Monkey {
    items_inspected: usize,
    items: VecDeque<u64>,
    operation: Expr,
    divisor: u64,
    targets: [usize; 2],
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_index, items_str, operation, divisor, target0, target1) = scan_fmt!(
            s,
            r#"Monkey {}:
                Starting items: {[ ,0-9]}
                Operation: new = {[ ,+*a-zA-Z0-9]}
                Test: divisible by {}
                    If true: throw to monkey {}
                    If false: throw to monkey {}
            "#,
            usize,
            String,
            Expr,
            u64,
            usize,
            usize
        )
        .unwrap();

        Ok(Self {
            items_inspected: 0,
            items: items_str
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation,
            divisor,
            targets: [target0, target1],
        })
    }
}

impl Monkey {
    fn throw_next_item(&mut self) -> Option<(usize, u64)> {
        let worry = self.items.pop_front()?;
        self.items_inspected += 1;
        let worry = self.operation.eval(worry);
        Some((
            if worry % self.divisor == 0 {
                self.targets[0]
            } else {
                self.targets[1]
            },
            worry,
        ))
    }
}

fn main() {
    let mut monkeys: Vec<_> = INPUT
        .split("\n\n")
        .map(|m| m.parse::<Monkey>().unwrap())
        .collect();
    let modulus: u64 = monkeys.iter().map(|m| m.divisor).product();
    for _round in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            while let Some((target, item)) = monkeys[monkey_idx].throw_next_item() {
                monkeys[target].items.push_back(item % modulus);
            }
        }
    }
    let monkey_business: usize = monkeys
        .iter()
        .map(|m| Reverse(m.items_inspected))
        .k_smallest(2)
        .map(|v| v.0)
        .product();
    println!("{}", monkey_business);
}
