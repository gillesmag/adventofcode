use std::fs;
use pathfinding::grid::Grid;
use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    //let filename = "test.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let weights: Vec<Vec<usize>> = file.lines().map(|line| {
        line.chars()
            .into_iter()
            .filter_map(|v| String::from(v).parse().ok())
            .collect::<Vec<usize>>()
    }).collect();

    let (width, height) = (weights[0].len(), weights.len());
    //println!("{:?} {:?}", width, height);
    let mut g = Grid::new(width, height);
    g.fill();

    let goal = (width-1, height-1);
    let result = dijkstra(&(0, 0), |p| {
        g.neighbours(*p).into_iter().map(|(x, y)| {
            ((x, y), weights[y][x])
        }).collect::<Vec<((usize, usize), usize)>>()
    }, |p| *p == goal);

    if let Some((_, risk)) = result {
        println!("{}", risk);
    }

    // build new grid for second part
    let mut g = Grid::new(width*5, height*5);
    g.fill();
    //println!("{:?}", g);

    let mut new_weights = vec![vec![0usize; width*5]; height*5];
    //println!("{:?}", new_weights);

    let coords: Vec<(usize, usize)> = (0..height).into_iter().flat_map(|row| {
        (0..width).into_iter().map(|col| (row, col)).collect::<Vec<(usize, usize)>>()
    }).collect();
    let tile_coords: Vec<(usize, usize)> = (0..5).into_iter().flat_map(|row| {
        (0..5).into_iter().map(|col| (row, col)).collect::<Vec<(usize, usize)>>()
    }).collect();
    for (x,y) in coords {
        for (tile_x, tile_y) in &tile_coords {
            let new_val = (weights[y][x] + tile_x + tile_y - 1) % 9 + 1;
            new_weights[y+tile_y*height][x+tile_x*width] = new_val;
            //println!("{} {} {}", tile_x, tile_y, new_weights[tile_y*height][tile_x*width]);
        }
    }

    let goal = (width*5-1, height*5-1);
    let result = dijkstra(&(0, 0), |p| {
        g.neighbours(*p).into_iter().map(|(x, y)| {
            ((x, y), new_weights[y][x])
        }).collect::<Vec<((usize, usize), usize)>>()
    }, |p| *p == goal);
 
    if let Some((_, risk)) = result {
        println!("{}", risk);
    }

    //for (tile_x, tile_y) in &tile_coords {
    //    println!("{} {} {}", tile_x, tile_y, new_weights[tile_y*height][tile_x*width]);
    //}
    //println!("{:?}", new_weights);
    //println!("{:?}", new_weights[0]);
}
