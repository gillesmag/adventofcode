fn bitmap_filter(readings: &mut Vec<Vec<u32>>, bit_criteria: fn(u32, u32) -> bool) -> Option<u32> {
    for column in 0..readings[0].len() {
        let rows = readings.iter().count() as u32;
        let count: u32 = readings.iter().map(|r| r[column]).sum();
        readings.retain(|row| row[column] == bit_criteria(count, rows - count) as u32);
        let rows = readings.iter().count() as u32;
        if rows == 1 {
            return Some(readings[0].iter().fold(0, |acc, x| acc * 2 + x));
        }
    }
    None
}

pub fn part_a(readings: Vec<Vec<u32>>) -> u32 {
    let rows = readings.len();
    let columns = readings[0].len();

    let counts = (0..columns)
        .into_iter()
        .map(|c| (0..rows).into_iter().map(|r| readings[r][c]).sum());

    let most_common_values = counts
        .clone()
        .map(|s: u32| (s > (rows as u32) / 2) as u32)
        .collect::<Vec<u32>>();

    let gamma = most_common_values.iter().fold(0, |acc, &x| acc * 2 + x);

    let epsilon = most_common_values
        .iter()
        .map(|&val| if val == 0 { 1 } else { 0 })
        .fold(0, |acc, x| acc * 2 + x);

    gamma * epsilon
}

pub fn part_b(readings: Vec<Vec<u32>>) -> Option<u32> {
    let o2_generator_rating = bitmap_filter(&mut readings.clone(), |ones, zeros| ones >= zeros);
    let co2_scrubber_rating = bitmap_filter(&mut readings.clone(), |ones, zeros| ones < zeros);
    match (o2_generator_rating, co2_scrubber_rating) {
        (Some(o2), Some(co2)) => Some(o2 * co2),
        _ => None,
    }
}

pub fn day03(input: &str) -> (String, String) {
    let lines: Vec<&str> = input.lines().into_iter().collect();

    let readings: Vec<Vec<u32>> = lines
        .clone()
        .into_iter()
        .map(|l| {
            l.chars()
                .filter_map(|n| n.to_string().parse().ok())
                .collect()
        })
        .collect();

    (
        part_a(readings.clone()).to_string(),
        part_b(readings.clone()).unwrap().to_string(),
    )
}
