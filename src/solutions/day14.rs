use std::collections::HashMap;

pub fn day14(input: &str) -> (String, String) {
    //let filename = "test.txt";
    let lines = input.lines().collect::<Vec<&str>>();

    let template = lines[0].clone().chars().collect::<Vec<char>>();
    let rules: HashMap<&str, char> = lines[2..]
        .into_iter()
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .map(|v| (v[0], v[1].chars().into_iter().nth(0).unwrap()))
        .collect();

    let mut counts: HashMap<String, usize> = rules
        .clone()
        .into_iter()
        .map(|(left, _)| (String::from(left), 0))
        .collect();

    let tuples = template
        .windows(2)
        .map(|v| v.iter().cloned().collect::<String>())
        .collect::<Vec<String>>();
    for tuple in tuples {
        counts.entry(tuple).and_modify(|v| *v += 1);
    }

    let mut new_counts: HashMap<String, usize> = HashMap::new();

    // Uncomment below for part 1 solution
    // let steps = 10;
    let steps = 40;
    for _step in 1..=steps {
        let keys = counts
            .clone()
            .into_iter()
            .filter(|(_, v)| *v > 0)
            .map(|(k, _)| k.clone())
            .collect::<Vec<String>>();
        for key in keys {
            let val = counts.get(&key).unwrap().clone();
            let right = rules.get(&key.as_str()).unwrap();
            let mut triple: Vec<char> = key.clone().chars().collect();
            triple.insert(1, *right);
            for win in triple.windows(2).map(|w| w.iter().collect::<String>()) {
                let w = new_counts.entry(win).or_insert(0);
                *w += val;
            }
        }
        counts = new_counts.clone();
        new_counts.clear();
    }

    let mut letters = counts
        .clone()
        .into_iter()
        .filter(|(_, v)| *v > 0)
        .flat_map(|(k, _)| k.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();
    letters.sort();
    letters.dedup();

    for letter in &letters {
        let double_letter: String = [*letter, *letter].iter().cloned().collect();
        counts.entry(double_letter).and_modify(|v| *v *= 2);
    }

    let all_counts: HashMap<char, usize> = letters
        .into_iter()
        .map(|l| {
            let s: usize = counts
                .clone()
                .into_iter()
                .filter(|(k, _)| k.contains(l))
                .map(|(_, v)| v)
                .sum();
            (l, if s % 2 == 0 { s / 2 } else { (s + 1) / 2 })
        })
        .collect();

    let min = all_counts.values().into_iter().min().unwrap();
    let max = all_counts.values().into_iter().max().unwrap();

    ((max - min).to_string(), "".to_string())
}
