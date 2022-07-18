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

fn parse(input: &str) -> (Vec<Vec<Vec<&str>>>, Vec<Vec<char>>) {
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

    (lines, perms)
}

fn part_a(lines: &Vec<Vec<Vec<&str>>>) -> u32 {
    let mut unique_counter = 0u32;

    for line in lines {
        let output = &line[1];
        let lengths = output.into_iter().map(|v| v.len()).collect::<Vec<usize>>();

        for length in lengths {
            if [2, 4, 3, 7].contains(&length) {
                unique_counter += 1;
            }
        }
    }

    unique_counter
}

fn part_b(lines: &Vec<Vec<Vec<&str>>>, perms: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut values: Vec<Option<usize>> = Vec::with_capacity(perms.len());

    for line in lines {
        let (patterns, output) = (&line[0], &line[1]);

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

        'outer: for perm in perms {
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

    total
}

pub fn day08(input: &str) -> (String, String) {
    let (lines, perms) = parse(input);
    (
        part_a(&lines).to_string(),
        part_b(&lines, &perms).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 8);
        let (lines, _) = parse(&input);
        assert_eq!(part_a(&lines), 26);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 8);
        let (lines, perms) = parse(&input);
        assert_eq!(part_b(&lines, &perms), 61229);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 8);
        let (lines, _) = parse(&input);
        assert_eq!(part_a(&lines), 456);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 8);
        let (lines, perms) = parse(&input);
        assert_eq!(part_b(&lines, &perms), 1091609);
    }
}
