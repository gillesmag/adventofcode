fn parse(input: &str) {
}

fn part_a(input: &str) -> usize {
    0
}

fn part_b(input: &str) -> usize {
    0
}

pub fn day04(input: &str) -> (String, String) {
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
        let input = read_file("examples", X);
        assert_eq!(part_a(parse(&input.unwrap())), X);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", X);
        assert_eq!(part_b(parse(&input.unwrap())), X);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", X);
        assert_eq!(part_a(parse(&input.unwrap())), X);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", X);
        assert_eq!(part_b(parse(&input.unwrap())), X);
    }
}
