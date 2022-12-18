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
    let voxels: HashSet<[i32; 3]> = INPUT
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();
            [x, y, z]
        })
        .collect();

    let (mut min_x, mut max_x) = voxels.iter().map(|v| v[0]).minmax().into_option().unwrap();
    let (mut min_y, mut max_y) = voxels.iter().map(|v| v[1]).minmax().into_option().unwrap();
    let (mut min_z, mut max_z) = voxels.iter().map(|v| v[2]).minmax().into_option().unwrap();
    min_x -= 1;
    min_y -= 1;
    min_z -= 1;
    max_x += 1;
    max_y += 1;
    max_z += 1;

    let is_outside = |&[x, y, z]: &[i32; 3]| {
        x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z
    };

    let mut outside = HashSet::new();
    let mut stack = vec![[min_x, min_y, min_z]];
    while let Some([x, y, z]) = stack.pop() {
        outside.insert([x, y, z]);
        for &[a, b, c] in &offsets {
            let pos = [x + a, y + b, z + c];
            if !voxels.contains(&pos) && !is_outside(&pos) && !outside.contains(&pos) {
                stack.push(pos);
            }
        }
    }

    let mut faces = 0;
    for &[x, y, z] in &outside {
        faces += 6 - offsets
            .iter()
            .filter(|&[a, b, c]| {
                let pos = [a + x, b + y, c + z];
                outside.contains(&pos) || is_outside(&pos)
            })
            .count();
    }
    println!("{}", faces);
}
