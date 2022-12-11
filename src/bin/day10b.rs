const INPUT: &str = include_str!("../../inputs/day10.txt");

struct Cpu {
    clock: i64,
    x: i64,
}

impl Cpu {
    fn new() -> Self {
        Self { clock: 0, x: 1 }
    }
    fn tick(&mut self) {
        let pos = self.clock % 40;
        self.clock += 1;
        if pos.abs_diff(self.x) <= 1 {
            print!("#");
        } else {
            print!(" ");
        }
        if pos == 39 {
            println!();
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
}
