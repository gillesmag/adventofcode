use std::collections::HashSet;

fn start(input: &str, size: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    let mut index = 0;
    for win in chars.windows(size) {
        let uniques: HashSet<char> = HashSet::from_iter(win.iter().copied());
        if uniques.len() == size {
            index += size;
            break;
        }
        index += 1;
    }
    index
}

fn part_a(input: &str) -> usize {
    start(input, 4)
}

fn part_b(input: &str) -> usize {
    start(input, 14)
}

pub fn day06(input: &str) -> (String, String) {
    (part_a(&input).to_string(), part_b(&input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let examples = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in examples {
            assert_eq!(part_a(input), expected);
        }
    }

    #[test]
    fn test_example_part_b() {
        let examples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expected) in examples {
            assert_eq!(part_b(input), expected);
        }
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 6);
        assert_eq!(part_a(&input.unwrap()), 1262);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 6);
        assert_eq!(part_b(&input.unwrap()), 3444);
    }
}
