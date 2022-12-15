use std::collections::HashSet;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day15.txt");

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    radius: u32,
}

impl Sensor {
    fn y_intersection(&self, y: i32) -> Option<(i32, i32)> {
        let y_dist = self.pos.1.abs_diff(y);
        if y_dist <= self.radius {
            let x_dist = (self.radius - y_dist) as i32;
            Some((self.pos.0 - x_dist, self.pos.0 + x_dist))
        } else {
            None
        }
    }
}

fn main() {
    let (sensors, beacons): (Vec<_>, HashSet<_>) = INPUT
        .lines()
        .map(|line| {
            let (a, b, c, d) = scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            (
                Sensor {
                    pos: (a, b),
                    radius: a.abs_diff(c) + b.abs_diff(d),
                },
                (c, d),
            )
        })
        .unzip();
    let y = 2000000;

    let mut intersections: Vec<_> = sensors
        .iter()
        .filter_map(|sensor| sensor.y_intersection(y))
        .flat_map(|(x0, x1)| [(x0, 1), (x1 + 1, -1)])
        .collect();
    intersections.sort();

    let mut last_x = 0;
    let mut accum = 0;
    let mut total = 0;
    for (x, s) in intersections {
        if accum == 0 {
            last_x = x;
        }
        accum += s;
        if accum == 0 {
            total += x - last_x;
        }
    }
    total -= beacons.iter().filter(|beacon| beacon.1 == y).count() as i32;

    println!("{}", total);
}
