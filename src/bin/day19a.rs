use std::collections::HashMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Debug)]
struct Blueprint {
    recipes: [[i32; 3]; 4],
}

type Input = ([i32; 3], [i32; 3], i32);

impl Blueprint {
    fn solve(
        &self,
        mut resources: [i32; 3],
        mut robots: [i32; 3],
        time: i32,
        new_robot: Option<usize>,
        cache: &mut HashMap<Input, i32>,
    ) -> i32 {
        for i in 0..3 {
            resources[i] += robots[i];
        }
        if let Some(robot) = new_robot {
            robots[robot] += 1;
        }
        if time == 0 {
            0
        } else {
            let cache_key = (resources, robots, time);
            if let Some(res) = cache.get(&cache_key) {
                return *res;
            }
            let mut best = 0;
            for i in (0..4).rev() {
                let recipe = self.recipes[i];
                let new_resources = [
                    resources[0] - recipe[0],
                    resources[1] - recipe[1],
                    resources[2] - recipe[2],
                ];
                if new_resources.iter().all(|&r| r >= 0) {
                    if i == 3 {
                        return self.solve(new_resources, robots, time - 1, None, cache)
                            + (time - 1);
                    } else {
                        best =
                            best.max(self.solve(new_resources, robots, time - 1, Some(i), cache));
                    }
                }
            }
            best = best.max(self.solve(resources, robots, time - 1, None, cache));
            cache.insert(cache_key, best);
            best
        }
    }
}

fn main() {
    let result: i32 = INPUT.lines().map(|line| {
        let (id, ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs) = scan_fmt!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", i32, i32, i32, i32, i32, i32, i32).unwrap();
        let bp = Blueprint {
            recipes: [
                [ore_ore, 0, 0],
                [clay_ore, 0, 0],
                [obs_ore, obs_clay, 0],
                [geo_ore, 0, geo_obs],
            ]
        };
        let num_geo = bp.solve([0,0,0], [1,0,0], 23, None, &mut HashMap::new());
        println!("{}", num_geo);
        num_geo * id
    }).sum();
    println!("{}", result);
}
