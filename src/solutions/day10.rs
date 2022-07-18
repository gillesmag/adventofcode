fn parse(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

fn process_line(line: &str) -> (Vec<char>, Option<char>) {
    let mut stack: Vec<char> = vec![];
    let mut non_matching = None;

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let previous = stack.pop().unwrap();
                let matching = match (previous, c) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
                    _ => false,
                };
                if !matching {
                    non_matching = Some(c);
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    (stack, non_matching)
}

fn part_a(lines: &Vec<&str>) -> u32 {
    let mut points = 0u32;

    for line in lines {
        let (_, non_matching) = process_line(line);

        if let Some(non_matching) = non_matching {
            points += match non_matching {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            };
        }
    }

    points
}

fn part_b(lines: &Vec<&str>) -> usize {
    let mut scores: Vec<usize> = vec![];

    for line in lines {
        let (stack, non_matching) = process_line(line);

        if non_matching.is_none() {
            scores.push(stack.iter().rev().fold(0, |acc, val| {
                acc * 5
                    + match val {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            }))
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub fn day10(input: &str) -> (String, String) {
    let lines = parse(input);
    (part_a(&lines).to_string(), part_b(&lines).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 10);
        let lines = parse(&input);
        assert_eq!(part_a(&lines), 26397);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 10);
        let lines = parse(&input);
        assert_eq!(part_b(&lines), 288957);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 10);
        let lines = parse(&input);
        assert_eq!(part_a(&lines), 364389);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 10);
        let lines = parse(&input);
        assert_eq!(part_b(&lines), 2870201088);
    }
}
