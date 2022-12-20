const INPUT: &str = include_str!("../../inputs/day20.txt");

fn main() {
    let mut numbers = INPUT
        .lines()
        .enumerate()
        .map(|(i, n)| (n.parse().unwrap(), i as i32))
        .collect::<Vec<(i32, _)>>();

    for i in 0..numbers.len() {
        let (n, old_pos) = numbers[i];
        let new_pos = (old_pos + n).rem_euclid(numbers.len() as i32 - 1);
        if new_pos < old_pos {
            for (_, pos) in &mut numbers {
                if *pos >= new_pos && *pos < old_pos {
                    *pos += 1;
                }
            }
        } else if new_pos > old_pos {
            for (_, pos) in &mut numbers {
                if *pos > old_pos && *pos <= new_pos {
                    *pos -= 1;
                }
            }
        }
        numbers[i].1 = new_pos;
    }

    numbers.sort_by_key(|(_, i)| *i);
    let zero_pos = numbers.iter().position(|(n, _)| *n == 0).unwrap();
    numbers.rotate_left(zero_pos);

    println!("{}", numbers[1000].0 + numbers[2000].0 + numbers[3000].0);
}
