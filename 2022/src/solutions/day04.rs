use itertools::Itertools;
use std::collections::HashSet;

type Sections = Vec<(HashSet<usize>, HashSet<usize>)>;

fn parse(input: &str) -> Sections {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|sections| {
                    let sec_tuple = sections
                        .split("-")
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap();

                    HashSet::from_iter((sec_tuple.0)..=(sec_tuple.1))
                })
                .collect_tuple::<(HashSet<usize>, HashSet<usize>)>()
                .unwrap()
        })
        .collect()
}

fn part_a(sections: Sections) -> usize {
    sections
        .into_iter()
        .filter(|(first, second)| first.is_subset(&second) || second.is_subset(&first))
        .count()
}

fn part_b(sections: Sections) -> usize {
    sections
        .into_iter()
        .filter(|(first, second)| !first.intersection(&second).next().is_none())
        .count()
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
        let input = read_file("examples", 4);
        assert_eq!(part_a(parse(&input)), 2);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 4);
        assert_eq!(part_b(parse(&input)), 4);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 4);
        assert_eq!(part_a(parse(&input)), 448);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 4);
        assert_eq!(part_b(parse(&input)), 794);
    }
}
