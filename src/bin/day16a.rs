use std::collections::{HashMap, HashSet, VecDeque};

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day16.txt");

#[derive(Debug)]
struct Valve {
    flow_rate: u64,
    tunnels: Vec<(usize, usize)>,
}

fn find_shortest_paths(from_id: usize, valves: &HashMap<usize, Valve>) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    queue.push_back((from_id, 0));
    while let Some((id, dist)) = queue.pop_front() {
        if !visited.insert(id) {
            continue;
        }
        let valve = &valves[&id];
        if dist > 0 && valve.flow_rate > 0 {
            result.push((id, dist));
        }
        for &(next_id, _) in &valve.tunnels {
            queue.push_back((next_id, dist + 1));
        }
    }
    result
}

fn walk_valves(
    id: usize,
    valves: &HashMap<usize, Valve>,
    visited: &mut HashSet<usize>,
    mut time_left: u64,
    mut base_flow: u64,
) -> u64 {
    if !visited.insert(id) {
        return 0;
    }
    let valve = &valves[&id];
    if valve.flow_rate > 0 {
        // Open valve
        time_left -= 1;
    }
    base_flow += valve.flow_rate * time_left;

    let mut best = base_flow;
    for &(next_id, dist) in &valve.tunnels {
        if time_left > dist as u64 {
            best = best.max(walk_valves(
                next_id,
                valves,
                visited,
                time_left - dist as u64,
                base_flow,
            ));
        }
    }
    visited.remove(&id);
    best
}

fn main() {
    let mut id_map = HashMap::new();
    let mut conv_id = |id_str: String| -> usize {
        let new_id = id_map.len();
        *id_map.entry(id_str).or_insert(new_id)
    };
    let valves: HashMap<_, _> = INPUT
        .lines()
        .map(|line| {
            let (id, flow_rate, tunnels_str) = scan_fmt!(
                line,
                "Valve {} has flow rate={d}; tunnels lead to valves {[A-Z, ]}",
                String,
                u64,
                String
            )
            .or_else(|_| {
                scan_fmt!(
                    line,
                    "Valve {} has flow rate={d}; tunnel leads to valve {[A-Z, ]}",
                    String,
                    u64,
                    String
                )
            })
            .unwrap();
            let tunnels = tunnels_str
                .split(", ")
                .map(|s| (conv_id(s.into()), 1))
                .collect();
            (conv_id(id), Valve { flow_rate, tunnels })
        })
        .collect();

    let start_id = id_map["AA"];
    let new_valves: HashMap<_, _> = valves
        .iter()
        .filter(|(&id, valve)| id == start_id || valve.flow_rate > 0)
        .map(|(&id, valve)| {
            (
                id,
                Valve {
                    flow_rate: valve.flow_rate,
                    tunnels: find_shortest_paths(id, &valves),
                },
            )
        })
        .collect();

    let mut visited = HashSet::new();
    let result = walk_valves(start_id, &new_valves, &mut visited, 30, 0);
    println!("{}", result);
}
