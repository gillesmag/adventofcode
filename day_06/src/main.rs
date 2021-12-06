use std::collections::HashMap;
use std::fs;
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

fn main() {
    let filename = "input.txt";
    //let filename = "test.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let lanternfish: Vec<u64> = file.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect();

    println!("Part A: {}", count_fish(&lanternfish, 80));
    println!("Part B: {}", count_fish(&lanternfish, 256));
}
