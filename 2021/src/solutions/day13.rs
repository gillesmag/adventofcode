use std::collections::HashSet;

type Instruction<'a> = (&'a str, usize);
type Coordinate = (usize, usize);

fn visualize_paper(coords: &HashSet<Coordinate>) -> Vec<&str> {
    let mut output: Vec<&str> = vec![];
    let max_x = coords.into_iter().map(|v| v.0).max().unwrap();
    let max_y = coords.into_iter().map(|v| v.1).max().unwrap();

    (0..=max_y).into_iter().for_each(|y| {
        let mut chars = (0..=max_x)
            .into_iter()
            .map(|x| if coords.contains(&(x, y)) { "█" } else { " " })
            .collect::<Vec<&str>>();
        chars.push("\n");
        output.append(&mut chars);
    });
    output
}

fn fold(coords: &mut Vec<Coordinate>, instructions: &Vec<Instruction>) -> HashSet<Coordinate> {
    let flipped_coordinate = |coord: usize, offset| {
        if coord > offset {
            offset - (coord - offset)
        } else {
            coord
        }
    };

    for (axis, offset) in instructions.into_iter() {
        for coord in coords.iter_mut() {
            *coord = match *axis {
                "x" => (flipped_coordinate(coord.0, *offset), coord.1),
                "y" => (coord.0, flipped_coordinate(coord.1, *offset)),
                _ => unreachable!(),
            };
        }
    }

    HashSet::from_iter(coords.iter().cloned())
}

fn parse(input: &str) -> (Vec<Coordinate>, Vec<Instruction>) {
    let mut lines = input.lines().collect::<Vec<&str>>();

    let coords_end = lines
        .iter()
        .position(|&l| l == "")
        .expect("File incorrectly formatted");
    let mut instructions = lines.split_off(coords_end);
    instructions.remove(0);

    let coords = lines
        .into_iter()
        .map(|v| {
            v.split(",")
                .filter_map(|c| c.parse().ok())
                .collect::<Vec<usize>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<Coordinate>>();

    let instructions = instructions
        .into_iter()
        .map(|v| {
            v.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect::<Vec<&str>>()
        })
        .map(|v| (v[0], v[1].parse::<usize>().unwrap()))
        .collect::<Vec<Instruction>>();

    (coords, instructions)
}

fn part_a(coords: &Vec<Coordinate>, instructions: &Vec<Instruction>) -> usize {
    let single_instruction = instructions.clone().into_iter().take(1).collect();
    fold(&mut coords.clone(), &single_instruction).len()
}

fn part_b(coords: &Vec<Coordinate>, instructions: &Vec<Instruction>) -> String {
    let coordinates = fold(&mut coords.clone(), &instructions);
    visualize_paper(&coordinates).join("")
}

pub fn day13(input: &str) -> (String, String) {
    let (coords, instructions) = parse(input);
    let mut newline = "\n".to_owned();
    let code = part_b(&coords, &instructions);
    newline.push_str(&code);
    (part_a(&coords, &instructions).to_string(), newline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 13);
        let (coords, instructions) = parse(&input);
        assert_eq!(part_a(&coords, &instructions), 17);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 13);
        let (coords, instructions) = parse(&input);
        assert_eq!(
            part_b(&coords, &instructions),
            "█████
█   █
█   █
█   █
█████
"
        );
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 13);
        let (coords, instructions) = parse(&input);
        assert_eq!(part_a(&coords, &instructions), 664);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 13);
        let (coords, instructions) = parse(&input);
        assert_eq!(
            part_b(&coords, &instructions),
            "████ ████   ██ █  █ ████ █    ███  █   
█    █       █ █ █     █ █    █  █ █   
███  ███     █ ██     █  █    ███  █   
█    █       █ █ █   █   █    █  █ █   
█    █    █  █ █ █  █    █    █  █ █   
████ █     ██  █  █ ████ ████ ███  ████
"
        );
    }
}
