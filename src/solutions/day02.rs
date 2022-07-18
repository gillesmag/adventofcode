fn part_a(lines: Vec<(&str, i32)>) -> i32 {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for line in lines {
        let (direction, amount) = line;

        if direction == "forward" {
            horizontal_pos += amount;
        } else if direction == "up" {
            vertical_pos -= amount;
        } else if direction == "down" {
            vertical_pos += amount;
        } else {
            panic!("Unsupported direction");
        }
    }

    horizontal_pos * vertical_pos
}

fn part_b(lines: Vec<(&str, i32)>) -> i32 {
    let mut aim = 0;
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for line in lines {
        let (direction, amount) = line;

        if direction == "forward" {
            horizontal_pos += amount;
            vertical_pos += aim * amount;
        } else if direction == "up" {
            aim -= amount;
        } else if direction == "down" {
            aim += amount;
        } else {
            panic!("Unsupported direction");
        }
    }

    horizontal_pos * vertical_pos
}

fn parse_lines(input: &str) -> Vec<(&str, i32)> {
    input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| line.split(" ").collect())
        .map(|l: Vec<&str>| (l[0], l[1].parse::<i32>().expect("error parsing int")))
        .collect::<Vec<(&str, i32)>>()
}

pub fn day02(input: &str) -> (String, String) {
    let lines = parse_lines(&input);
    (
        part_a(lines.clone()).to_string(),
        part_b(lines.clone()).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 2);
        let lines = parse_lines(&input);
        assert_eq!(part_a(lines), 150);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 2);
        let lines = parse_lines(&input);
        assert_eq!(part_b(lines), 900);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 2);
        let lines = parse_lines(&input);
        assert_eq!(part_a(lines), 2147104);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 2);
        let lines = parse_lines(&input);
        assert_eq!(part_b(lines), 2044620088);
    }
}
