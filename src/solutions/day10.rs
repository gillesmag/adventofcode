pub fn day10(input: &str) -> (String, String) {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut points = 0;
    let mut scores: Vec<usize> = vec![];

    for line in lines {
        let mut stack: Vec<char> = vec![];
        let mut corrupted = false;

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
                        points += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };
                        corrupted = true;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
        if stack.len() > 0 && !corrupted {
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

    (points.to_string(), scores[scores.len() / 2].to_string())
}
