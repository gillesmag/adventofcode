use std::fs;
use std::collections::HashMap;

type Coordinate = (u32, u32);

fn gen_positions(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    if start.0 == end.0 && start.1 != end.1 {
        let (min, max) = if start.1 < end.1 { (start.1, end.1) } else { (end.1, start.1) };
        return (min..=max).into_iter().map(|comp| (start.0, comp)).collect();
    } else if start.1 == end.1 && start.0 != end.0 {
        let (min, max) = if start.0 < end.0 { (start.0, end.0) } else { (end.0, start.0) };
        return (min..=max).into_iter().map(|comp| (comp, start.1)).collect();
    } else {
        let r1: Vec<u32> = if start.0 < end.0 { (start.0..=end.0).into_iter().collect() } else { (end.0..=start.0).rev().into_iter().collect() };
        let r2: Vec<u32> = if start.1 < end.1 { (start.1..=end.1).into_iter().collect() } else { (end.1..=start.1).rev().into_iter().collect() };
        return r1.into_iter().zip(r2.into_iter()).collect();
    }
}

pub fn day05() {
    //let filename = "test.txt";
    let filename = "src/inputs/day5.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");

    let coordinates: Vec<Vec<(u32, u32)>> = file
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let components: Vec<u32> = coords
                        .split(",")
                        .filter_map(|component| component.parse().ok())
                        .collect();
                    (components[0], components[1])
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect();

    let mut seen_coords: HashMap<Coordinate, usize> = HashMap::new();

    println!("{:?}", gen_positions((9,7), (7,9)));

    for pair in coordinates {
        let (start, end) = (pair[0], pair[1]);
        let positions = gen_positions(start, end);
        for position in positions {
            match seen_coords.get(&position) {
                Some(count) => seen_coords.insert(position, count+1),
                None => seen_coords.insert(position, 1)
            };
        }
    }

    println!("{:?}", seen_coords.values().filter(|&&val| val > 1).count());
}
