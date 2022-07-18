fn triangular_sum(diffs: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for &diff in diffs {
        sum += (diff * (diff + 1)) / 2;
    }
    sum
}

fn compute_fuel_usage(crabs: &Vec<u64>, optimal_pos: u64) -> u64 {
    let mut distances = vec![];

    for &crab in crabs {
        let distance = (crab as i64) - (optimal_pos as i64);
        distances.push(distance.abs() as u64);
    }

    distances.iter().sum()
}

// assumes values are sorted
fn median(values: &Vec<u64>) -> Option<u64> {
    if values.len() == 0 {
        return None;
    }

    Some(values[values.len() / 2])
}

pub fn day07(input: &str) -> (String, String) {
    let mut crabs: Vec<u64> = input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect();

    crabs.sort();

    let optimal_distance = median(&crabs).unwrap();
    let fuel_usage = compute_fuel_usage(&crabs, optimal_distance);

    let min_pos = crabs[0].clone();
    let max_pos = crabs.last().unwrap().clone();
    let mut fuel_sums = vec![];
    for pos in min_pos..=max_pos {
        let mut distances = vec![];
        for &crab in &crabs {
            let dist = (pos as i64) - (crab as i64);
            distances.push(dist.abs() as u64);
        }
        fuel_sums.push(triangular_sum(&distances));
    }

    (
        fuel_usage.to_string(),
        fuel_sums.iter().min().unwrap().to_string(),
    )
}
