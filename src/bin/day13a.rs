use serde::Deserialize;

const INPUT: &str = include_str!("../../inputs/day13.txt");

#[derive(Deserialize)]
#[serde(untagged)]
enum PacketData {
    Integer(i32),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Integer(a), Self::List(b)) => [Self::Integer(*a)].as_slice().cmp(b.as_slice()),
            (Self::List(a), Self::Integer(b)) => a.as_slice().cmp(&[Self::Integer(*b)].as_slice()),
        }
    }
}

impl Eq for PacketData {}
impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let result: usize = INPUT
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair_str)| {
            let (a_str, b_str) = pair_str.split_once('\n').unwrap();
            let a: PacketData = serde_json::from_str(a_str).unwrap();
            let b: PacketData = serde_json::from_str(b_str).unwrap();
            if a <= b {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();
    println!("{}", result);
}
