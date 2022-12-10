#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}


fn tick(instructions: &Vec<Instruction>) -> Vec<(usize, i64)> {
    let mut states = vec![];
    let mut register_x = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                states.push(register_x);
            },
            Instruction::Addx(val) => {
                states.push(register_x);
                states.push(register_x);
                register_x += val;
            }
        }
    }
    states.into_iter().enumerate().collect::<Vec<(usize, i64)>>()
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut instruction = line.split(" ");
            let cmd = instruction.nth(0).unwrap();

            match cmd {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(instruction.nth(0).unwrap().parse::<i64>().unwrap()),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect::<Vec<_>>()
}

fn part_a(instructions: &Vec<Instruction>) -> i64 {
    tick(&instructions)
        .iter()
        .map(|(cycle, val)| (cycle+1, val))
        .filter(|(cycle, _)| (20+cycle) % 40 == 0)
        .map(|(cycle, register)| (cycle as i64) * register)
        .sum()
}

fn print_crt(crt: &Vec<Vec<char>>) -> String {
    String::from_iter(crt.iter().map(|row| String::from_iter(row.iter()) + "\n"))
}

fn part_b(instructions: &Vec<Instruction>) -> String {
    let cycles = tick(&instructions);
    let mut crt = vec![vec!['.'; 40]; 6];
    let rows = cycles.chunks(40);
    for (current_row, row) in rows.enumerate() {
        let mut current_col = 0;
        for (_, value) in row {
            let v = (value + 1) as usize;
            if (current_col..=current_col+2).contains(&v) {
                crt[current_row][current_col] = '#';
            }
            current_col += 1;
        }
    }
    "\n".to_owned() + &print_crt(&crt)
}

pub fn day10(input: &str) -> (String, String) {
    (
        part_a(&parse(&input)).to_string(),
        part_b(&parse(&input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a_small() {
        let input = "noop\naddx 3\naddx -5";
        let result = tick(&parse(&input))[4];
        assert_eq!(result, (4, 4));
    }

    #[test]
    fn test_example_part_a_big() {
        let input = read_file("examples", 10);
        assert_eq!(part_a(&parse(&input.unwrap())), 13140);
    }

    #[test]
    fn test_example_part_b() {
        let expected = "\n\
##..##..##..##..##..##..##..##..##..##..\n\
###...###...###...###...###...###...###.\n\
####....####....####....####....####....\n\
#####.....#####.....#####.....#####.....\n\
######......######......######......####\n\
#######.......#######.......#######.....\n";
        let input = read_file("examples", 10);
        assert_eq!(part_b(&parse(&input.unwrap())), expected);
    }


    #[test]
    fn test_input_part_b() {
        let expected = "\n\
####.####.####.###..###...##..#..#.#....\n\
#.......#.#....#..#.#..#.#..#.#.#..#....\n\
###....#..###..#..#.#..#.#..#.##...#....\n\
#.....#...#....###..###..####.#.#..#....\n\
#....#....#....#....#.#..#..#.#.#..#....\n\
####.####.#....#....#..#.#..#.#..#.####.\n";
        let input = read_file("inputs", 10);
        assert_eq!(part_b(&parse(&input.unwrap())), expected);
    }
}
