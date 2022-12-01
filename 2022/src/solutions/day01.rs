use itertools::Itertools;

fn parse_input(lines: &str) -> Vec<Vec<usize>> {
    lines
        .split("\n\n")
        .map(|l|
             l.lines().map(|v| v.parse::<usize>().unwrap())
             .collect::<Vec<_>>()
        )
        .collect::<Vec<Vec<_>>>()
}

fn part_a(lines: &str) -> usize {
    parse_input(lines)
        .iter()
        .map(|snacks| snacks.iter().sum::<usize>())
        .max()
        .unwrap()
}

fn part_b(lines: &str) -> usize {
    parse_input(lines)
        .iter()
        .map(|snacks| snacks.iter().sum::<usize>())
        .sorted()
        .into_iter()
        .sorted()
        .rev()
        .take(3).sum::<usize>()
}

pub fn day01(input: &str) -> (String, String) {
    (
        part_a(input).to_string(),
        part_b(input).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 1);
        assert_eq!(part_a(&input), 24000);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 1);
        assert_eq!(part_b(&input), 45000);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 1);
        assert_eq!(part_a(&input), 74711);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 1);
        assert_eq!(part_b(&input), 209481);
    }
}
