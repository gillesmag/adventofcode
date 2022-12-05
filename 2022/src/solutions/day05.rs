type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

fn parse(input: &str) -> Input {
    let input = input.split("\n\n").collect::<Vec<_>>();

    let stacks = input[0]
        .lines()
        .filter(|line| !line.starts_with(" 1"))
        .map(|line| {
            line.chars()
                .skip(1)
                .enumerate()
                .filter(|&(i, _)| i % 4 == 0)
                .map(|(_, v)| v)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let stacks_transposed: Vec<Vec<char>> = (0..stacks[0].len())
        .map(|i| {
            stacks
                .iter()
                .map(|inner| inner[i].clone())
                .filter(|&v| v != ' ')
                .rev()
                .collect()
        })
        .collect();

    let instructions = input[1]
        .lines()
        .map(|line| {
            let components = line.split(" ").collect::<Vec<_>>();
            (
                components[1].parse::<usize>().unwrap(),
                components[3].parse::<usize>().unwrap() - 1,
                components[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<Vec<_>>();
    (stacks_transposed, instructions)
}

fn execute(input: Input, in_order: bool) -> String {
    let (stacks, instructions) = input;

    let mut stacks = stacks;

    for instruction in instructions {
        let mut crane_stack: Vec<char> = vec![];
        for _ in 0..instruction.0 {
            if let Some(elem) = stacks[instruction.1].pop() {
                crane_stack.push(elem);
            }
        }
        if !in_order {
            crane_stack = crane_stack.into_iter().rev().collect::<Vec<_>>();
        }
        for val in crane_stack {
            stacks[instruction.2].push(val);
        }
    }

    let top_elements = stacks.into_iter().map(|mut stack| stack.pop().unwrap());

    top_elements.collect::<String>()
}

fn part_a(input: Input) -> String {
    execute(input, true)
}

fn part_b(input: Input) -> String {
    execute(input, false)
}

pub fn day05(input: &str) -> (String, String) {
    (part_a(parse(&input.clone())), part_b(parse(&input.clone())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 5);
        assert_eq!(part_a(parse(&input.unwrap())), "CMZ");
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 5);
        assert_eq!(part_b(parse(&input.unwrap())), "MCD");
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 5);
        assert_eq!(part_a(parse(&input.unwrap())), "FZCMJCRHZ");
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 5);
        assert_eq!(part_b(parse(&input.unwrap())), "JSDHQMZGF");
    }
}
