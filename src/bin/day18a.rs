use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day18.txt");

fn main() {
    let offsets = [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ];
    let mut voxels = HashSet::<[i32; 3]>::new();
    let mut faces = 0;
    for line in INPUT.lines() {
        faces += 6;
        let (x, y, z) = line
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        voxels.insert([x, y, z]);
        faces -= offsets
            .iter()
            .filter(|&[a, b, c]| voxels.contains(&[a + x, b + y, c + z]))
            .count()
            * 2;
    }
    println!("{}", faces);
}
