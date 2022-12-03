use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .split("\n\n")
        .map(|l| l.lines().map(|v| v.parse::<usize>().unwrap()).sum())
}

fn part_a(lines: impl Iterator<Item = usize>) -> usize {
    lines.max().unwrap()
}

fn part_b(lines: impl Iterator<Item = usize>) -> usize {
    lines.sorted().rev().take(3).sum()
}

pub fn day01(input: &str) -> (String, String) {
    (
        part_a(parse(&input)).to_string(),
        part_b(parse(&input)).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = &read_file("examples", 1);
        assert_eq!(part_a(parse(input)), 24000);
    }

    #[test]
    fn test_example_part_b() {
        let input = &read_file("examples", 1);
        assert_eq!(part_b(parse(input)), 45000);
    }

    #[test]
    fn test_input_part_a() {
        let input = &read_file("inputs", 1);
        assert_eq!(part_a(parse(input)), 74711);
    }

    #[test]
    fn test_input_part_b() {
        let input = &read_file("inputs", 1);
        assert_eq!(part_b(parse(input)), 209481);
    }
}
