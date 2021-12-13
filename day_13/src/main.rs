use std::fs;
use std::collections::{HashMap, HashSet};

fn print_coords(coords: &Vec<Vec<usize>>) {
    let max_x = coords.into_iter().map(|v| v[0]).max().unwrap();
    let max_y = coords.into_iter().map(|v| v[1]).max().unwrap();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for coord in coords {
        let e = map.entry(coord[1]).or_insert(vec![]);
        e.push(coord[0]);
    }

    let mut sorted_keys = map.keys().collect::<Vec<&usize>>();
    sorted_keys.sort();

    for row in 0..=max_y {
        let e = map.get(&row);
        if e.is_none() {
            for col in 0..=max_x {
                print!(" ");
                print!(".");
            }
            println!("");
            continue;
        }
        let e  = e.unwrap();
        for col in 0..=max_x {
            if e.contains(&col) {
                //print!("#");
                print!("█");
            } else {
                //print!(".");
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    //let filename = "test.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");

    let mut lines = file.lines().collect::<Vec<&str>>();

    let coords_end = lines.iter().position(|&l| l == "");
    if coords_end.is_none() {
        panic!("File incorrectly formatted");
    }
    let coords_end = coords_end.unwrap();

    let mut instructions = lines.split_off(coords_end);
    let coords = lines;
    instructions.remove(0);

    let mut coords = coords
        .into_iter()
        .map(|v| {
            v.split(",")
                .filter_map(|c| c.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let instructions = instructions
        .into_iter()
        .map(|v| {
            v.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    //println!("{:?}", coords);
    //println!("{:?}", instructions);

    let mut new_coords: Vec<Vec<usize>> = vec![];

    for inst in instructions.into_iter() {
        let (axis, offset) = (inst[0], inst[1].parse::<usize>().unwrap());

        for coord in &coords {
            let mut new_coord = vec![0, 0usize];
            if axis == "x" {
                if coord[0] > offset {
                    new_coord[0] = offset - (coord[0] - offset);
                } else {
                    new_coord[0] = coord[0];
                }
                new_coord[1] = coord[1];
            } else if axis == "y" {
                if coord[1] > offset {
                    new_coord[1] = offset - (coord[1] - offset);
                } else {
                    new_coord[1] = coord[1];
                }
                new_coord[0] = coord[0];
            }
            new_coords.push(new_coord);
        }

        coords = new_coords.clone();
        new_coords.clear();
    }

    let mut unique_coords: HashSet<(usize, usize)> = HashSet::new();
    for coord in &coords {
        unique_coords.insert((coord[0], coord[1]));
    }
    print_coords(&coords);
    println!("{}", unique_coords.len());
    //print_coords(&new_coords);
}
