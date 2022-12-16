use std::time::Instant;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day15.txt");

struct Sensor {
    a: (i32, i32),
    b: (i32, i32),
}

fn convert_in(pos: (i32, i32)) -> (i32, i32) {
    (pos.0 - pos.1, pos.0 + pos.1)
}

fn convert_out(pos: (i32, i32)) -> (i32, i32) {
    ((pos.0 + pos.1) / 2, (pos.1 - pos.0) / 2)
}

fn check(sensors: &[Sensor], x: i32, y0: i32, y1: i32) -> Option<(i32, i32)> {
    if let Some((sensor, sensors)) = sensors.split_first() {
        if sensor.b.0 < x || sensor.a.0 > x || sensor.b.1 < y0 || sensor.a.1 > y1 {
            check(sensors, x, y0, y1)
        } else if sensor.b.1 >= y1 {
            if sensor.a.1 <= y0 {
                None
            } else {
                check(sensors, x, y0, sensor.a.1 - 1)
            }
        } else if sensor.a.1 <= y0 {
            check(sensors, x, sensor.b.1 + 1, y1)
        } else {
            check(sensors, x, y0, sensor.a.1 - 1).or_else(|| check(sensors, x, sensor.b.1 + 1, y1))
        }
    } else {
        Some((x, y0))
    }
}

fn main() {
    let start = Instant::now();
    let sensors: Vec<_> = INPUT
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
            let radius = (a.abs_diff(c) + b.abs_diff(d)) as i32;
            let pos = convert_in((a, b));
            Sensor {
                a: (pos.0 - radius, pos.1 - radius),
                b: (pos.0 + radius, pos.1 + radius),
            }
        })
        .collect();

    for sensor in sensors.iter() {
        let x = sensor.b.0 + 1;
        let y0 = sensor.a.1 - 1;
        let y1 = sensor.b.1 + 1;
        if let Some(res) = check(&sensors, x, y0, y1) {
            let pos = convert_out(res);
            if pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 4000000 && pos.1 <= 4000000 {
                println!("{}", (pos.0 as i64) * 4000000 + (pos.1 as i64));
                break;
            }
        }
    }
    println!("{:?}", start.elapsed());
}
