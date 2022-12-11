const INPUT: &str = include_str!("../../inputs/day10.txt");

struct Cpu {
    clock: i64,
    x: i64,
    signal_strength_sum: i64,
}

impl Cpu {
    fn new() -> Self {
        Self {
            clock: 0,
            x: 1,
            signal_strength_sum: 0,
        }
    }
    fn signal_strength(&self) -> i64 {
        self.clock * self.x
    }
    fn tick(&mut self) {
        self.clock += 1;
        if self.clock % 40 == 20 {
            self.signal_strength_sum += self.signal_strength();
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    for line in INPUT.lines() {
        let cmd_parts: Vec<_> = line.split(' ').collect();
        match cmd_parts.as_slice() {
            ["noop"] => cpu.tick(),
            ["addx", v] => {
                cpu.tick();
                cpu.tick();
                cpu.x += v.parse::<i64>().unwrap();
            }
            _ => panic!("Unknown command: {}", line),
        }
    }
    println!("{}", cpu.signal_strength_sum);
}
