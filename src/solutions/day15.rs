use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::grid::Grid;
use std::fs;

fn total_risk(weights: &Vec<Vec<usize>>) -> Option<usize> {
    let (width, height) = (weights[0].len(), weights.len());
    let mut g = Grid::new(width, height);
    g.fill();

    let goal = (width - 1, height - 1);
    let result = dijkstra(
        &(0, 0),
        |p| {
            g.neighbours(*p)
                .into_iter()
                .map(|(x, y)| ((x, y), weights[y][x]))
                .collect::<Vec<((usize, usize), usize)>>()
        },
        |p| *p == goal,
    );

    match result {
        Some((_, risk)) => Some(risk),
        _ => None
    }
}

pub fn day15() {
    //let filename = "test.txt";
    let filename = "src/inputs/day15.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let weights: Vec<Vec<usize>> = file
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter_map(|v| String::from(v).parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect();

    println!("{}", total_risk(&weights).unwrap());

    let (width, height) = (weights[0].len(), weights.len());
    let mut new_weights = vec![vec![0usize; width * 5]; height * 5];

    let coords: Vec<(usize, usize)> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .collect();
    let tile_coords: Vec<(usize, usize)> =
        (0..5).flat_map(|y| (0..5).map(move |x| (x, y))).collect();
    for &(x, y) in coords.iter() {
        for (tile_x, tile_y) in tile_coords.iter() {
            let new_val = (weights[y][x] + tile_x + tile_y - 1) % 9 + 1;
            new_weights[y + tile_y * height][x + tile_x * width] = new_val;
        }
    }

    println!("{}", total_risk(&new_weights).unwrap());
}
