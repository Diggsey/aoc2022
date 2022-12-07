use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day7.txt");

fn main() {
    let mut path = Vec::new();
    let mut dirs = HashMap::<_, u64>::new();
    for line in INPUT.lines() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if let Some(args) = cmd.strip_prefix("cd ") {
                if args == ".." {
                    path.pop();
                } else if args == "/" {
                    path.clear();
                } else {
                    path.push(args);
                }
            }
        } else {
            let (size_str, _name) = line.split_once(" ").unwrap();
            if size_str != "dir" {
                let size: u64 = size_str.parse().unwrap();
                for i in 0..=path.len() {
                    *dirs.entry(path[0..i].to_vec()).or_default() += size;
                }
            }
        }
    }
    let total: u64 = dirs.values().copied().filter(|v| *v <= 100000).sum();
    println!("{}", total);
}
