#[derive(Debug, Clone)]
struct Monkey<'a> {
    operation: Vec<&'a str>,
    test: u64,
    branch_result: (usize, usize)
}

fn parse(input: &str) -> Vec<(Vec<u64>, Monkey)> {
    input.split("\n\n").map(|group| {
        let mut lines = group.split("\n");
        let _monkey_id = lines.next();
        let starting_items_line = lines.next().unwrap();
        let items = starting_items_line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap()).collect::<Vec<_>>();

        let operation_line = lines.next().unwrap();
        let op_statement = operation_line
            .split(": ")
            .nth(1)
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .collect::<Vec<_>>();

        let test_line = lines.next();
        let divisible_by = test_line
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .nth(2)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let true_branch = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .nth(3)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_branch = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .nth(3)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        (items, Monkey {
            operation: op_statement,
            test: divisible_by,
            branch_result: (true_branch, false_branch)
        })
    }).collect::<Vec<_>>()
}

fn part_a(monkeys: Vec<(Vec<u64>, Monkey)>) -> usize {
    let mut items = monkeys.iter().map(|m| m.0.clone()).collect::<Vec<_>>();
    let monkeys = monkeys.iter().map(|m| m.1.clone()).collect::<Vec<_>>();
    let mut inspections = vec![0; monkeys.len()];
    for _round in 0..20 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            let mut new_items = items.clone();
            for item in &items[idx] {
                inspections[idx] += 1;
                new_items[idx].remove(0);
                let lhs = match monkey.operation[0] {
                    "old" => *item,
                    l => l.parse::<u64>().unwrap(),
                };

                let rhs = match monkey.operation[2] {
                    "old" => *item,
                    r => r.parse::<u64>().unwrap(),
                };

                let result = match monkey.operation[1] {
                    "+" => lhs + rhs,
                    "*" => lhs * rhs,
                    _ => panic!("unknown operator"),
                } / 3;

                if result % monkey.test == 0 {
                    new_items[monkey.branch_result.0].push(result);
                } else {
                    new_items[monkey.branch_result.1].push(result);
                }
            }
            items = new_items;
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).fold(1, |acc, x| acc * x)
}

fn part_b(monkeys: Vec<(Vec<u64>, Monkey)>) -> usize {
    let mut items = monkeys.iter().map(|m| m.0.clone()).collect::<Vec<_>>();
    let monkeys = monkeys.iter().map(|m| m.1.clone()).collect::<Vec<_>>();
    let mut inspections = vec![0; monkeys.len()];
    let max_div = monkeys.iter().map(|m| m.test).product::<u64>();
    for _round in 0..10000 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            let mut new_items = items.clone();
            for item in &items[idx] {
                inspections[idx] += 1;
                new_items[idx].remove(0);
                let lhs = match monkey.operation[0] {
                    "old" => *item,
                    l => l.parse::<u64>().unwrap(),
                };

                let rhs = match monkey.operation[2] {
                    "old" => *item,
                    r => r.parse::<u64>().unwrap(),
                };

                let result = match monkey.operation[1] {
                    "+" => lhs + rhs,
                    "*" => lhs * rhs,
                    _ => panic!("unknown operator"),
                } % max_div;

                if result % monkey.test == 0 {
                    new_items[monkey.branch_result.0].push(result);
                } else {
                    new_items[monkey.branch_result.1].push(result);
                }
            }
            items = new_items;
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).fold(1, |acc, x| acc * x)
}

pub fn day11(input: &str) -> (String, String) {
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
        let input = read_file("examples", 11);
        assert_eq!(part_a(parse(&input.unwrap())), 10605);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 11);
        assert_eq!(part_b(parse(&input.unwrap())), 2713310158);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 11);
        assert_eq!(part_a(parse(&input.unwrap())), 108240);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 11);
        assert_eq!(part_b(parse(&input.unwrap())), 25712998901);
    }
}
