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

fn parse(input: &str) -> Vec<u64> {
    let mut crabs: Vec<u64> = input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect();
    crabs.sort();
    crabs
}

fn part_a(crabs: &Vec<u64>) -> u64 {
    let optimal_distance = median(&crabs).unwrap();
    compute_fuel_usage(&crabs, optimal_distance)
}

fn part_b(crabs: &Vec<u64>) -> u64 {
    let min_pos = crabs[0].clone();
    let max_pos = crabs.last().unwrap().clone();
    let mut fuel_sums = vec![];
    for pos in min_pos..=max_pos {
        let mut distances = vec![];
        for &crab in crabs {
            let dist = (pos as i64) - (crab as i64);
            distances.push(dist.abs() as u64);
        }
        fuel_sums.push(triangular_sum(&distances));
    }
    *fuel_sums.iter().min().unwrap()
}

pub fn day07(input: &str) -> (String, String) {
    let crabs = parse(input);
    (part_a(&crabs).to_string(), part_b(&crabs).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 7);
        let crabs = parse(&input);
        assert_eq!(part_a(&crabs), 37);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 7);
        let crabs = parse(&input);
        assert_eq!(part_b(&crabs), 168);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 7);
        let crabs = parse(&input);
        assert_eq!(part_a(&crabs), 328262);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 7);
        let crabs = parse(&input);
        assert_eq!(part_b(&crabs), 90040997);
    }
}
