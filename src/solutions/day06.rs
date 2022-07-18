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

pub fn day06(input: &str) -> (String, String) {
    let lanternfish: Vec<u64> = input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect();

    (
        count_fish(&lanternfish, 80).to_string(),
        count_fish(&lanternfish, 256).to_string(),
    )
}
