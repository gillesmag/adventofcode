use std::collections::HashMap;
use std::mem;

fn count_fish(lanternfish: &Vec<u64>, days: u32) -> u64 {
    let mut previous_day = HashMap::new();
    let mut new_day = HashMap::new();

    for &fish in lanternfish {
        let count = previous_day.entry(fish).or_insert(0);
        *count += 1;
    }

    for _day in 0..days {
        for key in (0..=8).rev() {
            if let Some(count) = previous_day.get(&key) {
                let c = count.clone();
                if key == 0 {
                    new_day.entry(6).and_modify(|e| *e += c).or_insert(c);
                    new_day.entry(8).or_insert(c);
                } else {
                    new_day.entry(key - 1).or_insert(c);
                }
            }
        }
        mem::swap(&mut previous_day, &mut new_day);
        new_day.clear();
    }
    previous_day
        .into_values()
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect()
}

fn part_a(lanternfish: &Vec<u64>) -> u64 {
    count_fish(&lanternfish, 80)
}

fn part_b(lanternfish: &Vec<u64>) -> u64 {
    count_fish(&lanternfish, 256)
}

pub fn day06(input: &str) -> (String, String) {
    let lanternfish = parse(input);
    (
        part_a(&lanternfish).to_string(),
        part_b(&lanternfish).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 6);
        let lanternfish = parse(&input);
        assert_eq!(part_a(&lanternfish), 5934);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 6);
        let lanternfish = parse(&input);
        assert_eq!(part_b(&lanternfish), 26984457539);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 6);
        let lanternfish = parse(&input);
        assert_eq!(part_a(&lanternfish), 345793);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 6);
        let lanternfish = parse(&input);
        assert_eq!(part_b(&lanternfish), 1572643095893);
    }
}
