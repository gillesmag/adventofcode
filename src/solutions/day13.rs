use std::collections::HashSet;

type Instruction<'a> = (&'a str, usize);
type Coordinate = (usize, usize);

fn print_coords(coords: &HashSet<Coordinate>) {
    let max_x = coords.into_iter().map(|v| v.0).max().unwrap();
    let max_y = coords.into_iter().map(|v| v.1).max().unwrap();

    (0..=max_y).into_iter().for_each(|y| {
        (0..=max_x)
            .into_iter()
            .map(|x| if coords.contains(&(x, y)) { "█" } else { " " })
            .for_each(|c| print!("{}", c));
        println!("");
    });
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

pub fn day13(input: &str) -> (String, String) {
    //let filename = "test.txt";

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

    // part A
    let single_instruction = instructions.clone().into_iter().take(1).collect();
    let new_coords = fold(&mut coords.clone(), &single_instruction);

    // part B
    let new_coords = fold(&mut coords.clone(), &instructions);
    print_coords(&new_coords);

    (new_coords.len().to_string(), "".to_string())
}
