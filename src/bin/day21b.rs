use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day21.txt");

enum Monkey {
    Const(i64),
    BinOp(BinOp),
}

struct BinOp {
    a: &'static str,
    b: &'static str,
    op: &'static str,
}

impl Monkey {
    fn eval(
        &self,
        name: &'static str,
        monkeys: &HashMap<&'static str, Monkey>,
        values: &mut HashMap<&'static str, i64>,
    ) -> i64 {
        if let Some(value) = values.get(name) {
            *value
        } else {
            let value = match self {
                Self::Const(v) => *v,
                Self::BinOp(bin_op) => {
                    let av = monkeys[bin_op.a].eval(bin_op.a, monkeys, values);
                    let bv = monkeys[bin_op.b].eval(bin_op.b, monkeys, values);
                    match bin_op.op {
                        "+" => av + bv,
                        "-" => av - bv,
                        "*" => av * bv,
                        "/" => av / bv,
                        _ => panic!("Unknown op: {}", bin_op.op),
                    }
                }
            };
            values.insert(name, value);
            value
        }
    }
}

fn main() {
    let monkeys: HashMap<_, _> = INPUT
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(": ").unwrap();
            (
                name,
                if let Some((a, mut op, b)) = rest.split_ascii_whitespace().collect_tuple() {
                    if name == "root" {
                        op = "-";
                    }
                    Monkey::BinOp(BinOp { a, b, op })
                } else {
                    Monkey::Const(rest.parse().unwrap())
                },
            )
        })
        .collect();

    let mut minv = 0;
    let mut maxv = 4000000000000i64;
    loop {
        let v = minv + (maxv - minv) / 2;
        let mut values = HashMap::new();
        values.insert("humn", v);
        let res = monkeys["root"].eval("root", &monkeys, &mut values);
        if res < 0 {
            maxv = v - 1;
        } else if res > 0 {
            minv = v + 1;
        } else {
            for v in minv..=maxv {
                let mut values = HashMap::new();
                values.insert("humn", v);
                let res = monkeys["root"].eval("root", &monkeys, &mut values);
                if res == 0 {
                    println!("{}", v);
                    break;
                }
            }
            break;
        }
    }
}
