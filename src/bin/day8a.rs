use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day8.txt");

fn walk_trees(set: &mut HashSet<usize>, it: impl Iterator<Item = (usize, u32)>) {
    let mut last_height = None;
    for (pos, height) in it {
        if Some(height) > last_height {
            last_height = Some(height);
            set.insert(pos);
        }
    }
}

fn main() {
    let digits: Vec<_> = INPUT
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .collect();
    let width = INPUT.find("\n").unwrap();
    let height = digits.len() / width;
    let mut set = HashSet::new();

    for x in 0..width {
        let it = digits.iter().copied().skip(x).step_by(width);
        walk_trees(&mut set, it.clone());
        walk_trees(&mut set, it.rev());
    }
    for y in 0..height {
        let it = digits.iter().copied().skip(y * width).take(width);
        walk_trees(&mut set, it.clone());
        walk_trees(&mut set, it.rev());
    }
    println!("{}", set.len());
}
