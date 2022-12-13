use serde::Deserialize;

const INPUT: &str = include_str!("../../inputs/day13.txt");

#[derive(Deserialize, Clone)]
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
    let divisors = [
        PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]),
        PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]),
    ];
    let mut packets: Vec<_> = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<PacketData>(line).unwrap())
        .chain(divisors.iter().cloned())
        .collect();
    packets.sort();

    let result: usize = divisors
        .iter()
        .map(|divisor| packets.iter().position(|x| x == divisor).unwrap() + 1)
        .product();

    println!("{}", result);
}
