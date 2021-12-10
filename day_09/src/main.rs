use std::collections::HashMap;
use std::fs;

type Grid = Vec<Vec<usize>>;
type Coordinate = (usize, usize);

fn adjacent_positions(pos: Coordinate, bounds: Coordinate) -> Vec<Coordinate> {
    let mut positions = vec![];

    if pos.0 >= 1 {
        positions.push((pos.0 - 1, pos.1));
    }

    if pos.0 + 1 < bounds.0 {
        positions.push((pos.0 + 1, pos.1));
    }

    if pos.1 >= 1 {
        positions.push((pos.0, pos.1 - 1));
    }

    if pos.1 + 1 < bounds.1 {
        positions.push((pos.0, pos.1 + 1));
    }

    positions
}

fn part_1(grid: &Grid) -> usize {
    let bounds = (grid[0].len(), grid.len());

    let mut low_points: Vec<usize> = vec![];

    for (x, y) in (0..bounds.1).flat_map(|y| (0..bounds.0).map(move |x| (x, y))) {
        let value = grid[y][x];
        let adj = adjacent_positions((x, y), bounds);

        let min_val = adj.into_iter().map(|(x, y)| grid[y][x]).min().unwrap();
        if value < min_val {
            low_points.push(value);
        }
    }
    low_points.into_iter().map(|v| v + 1).sum::<usize>()
}

fn print_grid(grid: &Vec<Vec<Option<usize>>>) {
    let (rows, columns) = (grid.len(), grid[0].len());

    for y in 1..rows - 1 {
        for x in 1..columns - 1 {
            if let Some(cell) = grid[y][x] {
                print!("{}", cell);
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn part_2(grid: &Grid) -> usize {
    let bounds = (grid[0].len(), grid.len());

    println!("{:?}", bounds);

    let mut padded_grid = vec![vec![9; bounds.0 + 2]; bounds.1 + 2];

    for (y, row) in grid.into_iter().enumerate() {
        for (x, &cell) in row.into_iter().enumerate() {
            padded_grid[y + 1][x + 1] = cell;
        }
    }

    let mut count_grid: Vec<Vec<Option<usize>>> = vec![vec![None; bounds.0 + 2]; bounds.1 + 2];

    let mut counter = 0usize;

    for y in 1..=bounds.1 {
        for x in 1..=bounds.0 {
            if padded_grid[y][x] == 9 {
                continue;
            }
            let adj = adjacent_positions((x, y), (bounds.0 + 1, bounds.1 + 1));
            //print!("{} ", padded_grid[y][x]);
            // 1. set value for myself from neighbours (if any, otherwise set to counter)
            // 2. forward value to neighbours if possible
            let mut neighbours = adj
                .iter()
                .filter_map(|&(x, y)| count_grid[y][x])
                .collect::<Vec<usize>>();
            neighbours.sort();
            neighbours.dedup();
            if neighbours.len() == 0 {
                count_grid[y][x] = Some(counter);
                counter += 1;
            } else if neighbours.len() == 1 {
                count_grid[y][x] = Some(neighbours[0]);
            } else {
                // fix neighbours
                let min_neighbour = neighbours.iter().min().unwrap();
                let indexed_neighbours = adj
                    .iter()
                    .filter_map(|&(x, y)| {
                        if count_grid[y][x].is_some() {
                            Some(((x, y), count_grid[y][x].unwrap()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<((usize, usize), usize)>>();
                for ((nx, ny), val) in indexed_neighbours {
                    if val != *min_neighbour {
                        count_grid[ny][nx] = Some(*min_neighbour);
                    }
                }
                count_grid[y][x] = Some(*min_neighbour);
                //print_grid(&count_grid);
                //println!("{:?}", (x, y));
                //panic!("{:?}", neighbours);
            }
            let indexed_neighbours = adj
                .iter()
                .map(|&(x, y)| ((x, y), padded_grid[y][x]))
                .collect::<Vec<((usize, usize), usize)>>();
            for &((nx, ny), neighbour) in &indexed_neighbours {
                if neighbour == 9 {
                    continue;
                }

                padded_grid[ny][nx] = padded_grid[y][x];
            }
            //println!("{:?}", indexed_neighbours);
            //println!("{:?}", count_grid);
        }
    }
    print_grid(&count_grid);

    let mut totals: HashMap<usize, usize> = HashMap::new();
    for y in 1..=bounds.1 {
        for x in 1..=bounds.0 {
            if let Some(val) = count_grid[y][x] {
                totals.entry(val).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    let mut biggest = totals.values().into_iter().collect::<Vec<&usize>>();
    biggest.sort();
    biggest
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |acc, val| acc * val)
}

fn main() {
    //let filename = "test.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");

    let grid: Vec<Vec<usize>> = file
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|v| String::from(v).parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect();

    println!("{}", part_1(&grid));
    println!("{}", part_2(&grid));
}
