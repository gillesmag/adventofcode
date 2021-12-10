use std::collections::VecDeque;
use std::fs;

fn main() {
    //let filename = "test.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let lines = file.lines().collect::<Vec<&str>>();

    let mut points = 0;
    let mut incomplete_lines: Vec<VecDeque<char>> = vec![];

    for line in lines {
        let mut q: VecDeque<char> = VecDeque::new();
        let mut incorrect = false;

        for c in line.chars() {
            if ![')', '}', ']', '>'].contains(&c) {
                q.push_back(c);
                continue;
            }
            if let Some(last) = q.back() {
                if (*last == '(' && c == ')')
                    || (*last == '{' && c == '}')
                    || (*last == '[' && c == ']')
                    || (*last == '<' && c == '>')
                {
                    q.pop_back();
                } else {
                    points += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                    //println!("Error: {} {}", c, last);
                    //println!("{}", c);
                    //println!("{}", line);
                    //for i in 0..idx {
                    //    print!(" ");
                    //}
                    //println!("^");
                    incorrect = true;
                    break;
                }
            }
        }
        if q.len() > 0 && !incorrect {
            incomplete_lines.push(q.clone());
        }
    }

    println!("{}", points);

    //println!("{:?}", incomplete_lines.len());

    let mut scores: Vec<usize> = vec![];

    for incomplete in &mut incomplete_lines {
        //println!("{:?}", incomplete);

        let mut score = 0;
        loop {
            let last = incomplete.pop_back();
            if let Some(c) = last {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
            } else {
                break;
            }
        }
        scores.push(score);
    }

    scores.sort();

    println!("{:?}", scores[scores.len() / 2]);
}
