use itertools::Itertools;

fn parse_segment(positions: &Vec<usize>) -> Option<usize> {
    let segment_positions: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1
        vec![0, 2, 3, 4, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6],    // 9
    ];
    for (num, segment_configuration) in segment_positions.into_iter().enumerate() {
        if segment_configuration == *positions {
            return Some(num);
        }
    }
    None
}

pub fn day08(input: &str) -> (String, String) {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|part| part.split(" ").collect::<Vec<&str>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let perms = ('a'..='g')
        .into_iter()
        .permutations(7)
        .collect::<Vec<Vec<char>>>();

    let mut unique_counter = 0;
    let mut total = 0;

    let mut values: Vec<Option<usize>> = Vec::with_capacity(perms.len());

    for line in &lines {
        let (patterns, output) = (&line[0], &line[1]);
        let lengths = output.into_iter().map(|v| v.len()).collect::<Vec<usize>>();

        for length in lengths {
            if [2, 4, 3, 7].contains(&length) {
                unique_counter += 1;
            }
        }
        let sorted_patterns = patterns
            .into_iter()
            .map(|val| {
                let mut vals = val.chars().collect::<Vec<char>>();
                vals.sort();
                vals
            })
            .collect::<Vec<Vec<char>>>();

        let output_val = output
            .into_iter()
            .map(|val| {
                let mut vals = val.chars().collect::<Vec<char>>();
                vals.sort();
                vals
            })
            .collect::<Vec<Vec<char>>>();

        'outer: for perm in &perms {
            values.clear();
            for pattern in patterns {
                let mut positions = pattern
                    .chars()
                    .filter_map(|val| perm.iter().position(|&x| val == x))
                    .collect::<Vec<usize>>();
                positions.sort();
                let segment_value = parse_segment(&positions);
                if segment_value.is_none() {
                    continue 'outer;
                }
                values.push(segment_value);
            }
            if values.iter().all(|v| v.is_some()) {
                let output_val = output_val
                    .into_iter()
                    .filter_map(|val| sorted_patterns.iter().position(|v| val == *v))
                    .filter_map(|idx| values[idx])
                    .fold(0, |acc, val| acc * 10 + val);
                total += output_val;
                break;
            }
        }
    }

    (unique_counter.to_string(), total.to_string())
}
