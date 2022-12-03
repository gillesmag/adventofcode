use std::collections::HashSet;

fn priority(value: char) -> usize {
    if value.is_lowercase() {
        (value as usize) - ('a' as usize) + 1
    } else {
        (value as usize) - ('A' as usize) + 27
    }
}

fn part_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line_iter = line.chars().into_iter();
            let first_half = line_iter.clone().take(line.len() / 2);
            let second_half = line_iter.skip(line.len() / 2).take(line.len() / 2);

            let first_half: HashSet<char> = HashSet::from_iter(first_half);
            let second_half: HashSet<char> = HashSet::from_iter(second_half);

            let intersection = *first_half.intersection(&second_half).last().unwrap();

            priority(intersection)
        })
        .sum()
}

fn part_b(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            priority(
                group
                    .iter()
                    .map(|ln| ln.chars().collect::<HashSet<char>>())
                    .reduce(|acc, set| acc.intersection(&set).copied().collect())
                    .unwrap()
                    .into_iter()
                    .last()
                    .unwrap(),
            )
        })
        .sum::<usize>()
}

pub fn day03(input: &str) -> (String, String) {
    (part_a(&input).to_string(), part_b(&input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 3);
        assert_eq!(part_a(&input), 157);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 3);
        assert_eq!(part_b(&input), 70);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 3);
        assert_eq!(part_a(&input), 7793);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 3);
        assert_eq!(part_b(&input), 2499);
    }
}
