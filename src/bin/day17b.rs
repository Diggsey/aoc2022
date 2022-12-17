use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day17.txt");

type Rock = &'static [[i32; 2]];
type Map = HashSet<[i32; 2]>;

static ROCKS: [Rock; 5] = [
    &[[0, 0], [1, 0], [2, 0], [3, 0]],
    &[[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]],
    &[[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
    &[[0, 0], [0, 1], [0, 2], [0, 3]],
    &[[0, 0], [1, 0], [0, 1], [1, 1]],
];

fn is_solid(map: &Map, pos: [i32; 2]) -> bool {
    pos[0] <= 0 || pos[0] > 7 || pos[1] <= 0 || map.contains(&pos)
}

fn rock_can_exist(map: &Map, rock: Rock, pos: [i32; 2]) -> bool {
    rock.iter()
        .all(|part| !is_solid(map, [pos[0] + part[0], pos[1] + part[1]]))
}

fn place_rock(map: &mut Map, rock: Rock, pos: [i32; 2], max_height: &mut i32) {
    for part in rock {
        let new_pos = [pos[0] + part[0], pos[1] + part[1]];
        if new_pos[1] > *max_height {
            *max_height = new_pos[1];
        }
        map.insert(new_pos);
    }
}

fn main() {
    let mut map = Map::new();
    let mut max_height = 0;

    let mut jets = INPUT.trim().chars().cycle();

    let target: i64 = 1000000000000;
    let unique_length = 10;
    let cycle_length = 1745;
    let cycle_height = 2778;
    let remainder = (target - unique_length) % cycle_length;
    let cycles = (target - unique_length) / cycle_length;

    for &rock in ROCKS
        .iter()
        .cycle()
        .take((remainder + unique_length) as usize)
    {
        let mut pos = [3, max_height + 4];
        loop {
            let new_pos = match jets.next().unwrap() {
                '>' => [pos[0] + 1, pos[1]],
                '<' => [pos[0] - 1, pos[1]],
                c => panic!("Unexpected char: {c}"),
            };
            if rock_can_exist(&map, rock, new_pos) {
                pos = new_pos;
            }
            let new_pos = [pos[0], pos[1] - 1];
            if rock_can_exist(&map, rock, new_pos) {
                pos = new_pos;
            } else {
                break;
            }
        }
        place_rock(&mut map, rock, pos, &mut max_height);
    }

    println!("{}", (max_height as i64) + cycles * cycle_height);
}
