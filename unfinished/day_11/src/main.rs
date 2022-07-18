use std::fs;
use std::fmt::Debug;
use std::string::ToString;

use colored::*;

type Grid<T> = Vec<Vec<T>>;
type Coordinate = (usize, usize);

fn print_grid<T: ToString + Debug>(grid: &Grid<T>) {
    let (rows, columns) = (grid.len(), grid[0].len());

    for y in 0..rows {
        for x in 0..columns {
            //if x > 0 {
            //    print!(" ");
            //}
            let val = grid[y][x].to_string();
            if val == "0" {
                print!("{}", val.cyan());
            } else {
                print!("{}", val);
            }
        }
        println!("");
    }
}

fn adjacent_positions(pos: Coordinate, bounds: Coordinate) -> Vec<Coordinate> {
    let mut positions = vec![];

    if pos.0 >= 1 {
        if pos.1 >= 1 {
            positions.push((pos.0 - 1, pos.1 - 1));
        }

        positions.push((pos.0 - 1, pos.1));

        if pos.1 + 1 < bounds.1 {
            positions.push((pos.0 - 1, pos.1 + 1));
        }
    }

    if pos.0 + 1 < bounds.0 {
        if pos.1 >= 1 {
            positions.push((pos.0 + 1, pos.1 - 1));
        }

        positions.push((pos.0 + 1, pos.1));

        if pos.1 + 1 < bounds.1 {
            positions.push((pos.0 + 1, pos.1 + 1));
        }
    }

    if pos.1 >= 1 {
        positions.push((pos.0, pos.1 - 1));
    }

    if pos.1 + 1 < bounds.1 {
        positions.push((pos.0, pos.1 + 1));
    }

    positions
}

fn flash(grid: &mut Grid<usize>, flashed: &mut Grid<bool>, (x, y): Coordinate) -> bool {
    if grid[y][x] < 9 || flashed[y][x] {
        return false;
    }

    let mut updated = true;
    flashed[y][x] = true;
    for (x, y) in adjacent_positions((x, y), (grid[0].len(), grid.len())) {
        grid[y][x] += 1;
        updated = updated || flash(grid, flashed, (x, y));
    }
    updated
}

fn main() {
    //let filename = "test2.txt";
    let filename = "larger.txt";
    //let filename = "small.txt";
    let file = fs::read_to_string(filename).expect("Unable to read file");

    let steps = 2;

    let mut grid: Vec<Vec<usize>> = file
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|v| String::from(v).parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let bounds = (grid[0].len(), grid.len());
    let (rows, columns) = bounds;

    print_grid(&grid);

    for step in 1..=steps {
        println!("Step {}", step);
        let mut flashed: Vec<Vec<bool>> = (0..grid.len())
            .into_iter()
            .map(|_| {
                (0..grid[0].len())
                    .into_iter()
                    .map(|_| false)
                    .collect::<Vec<bool>>()
            })
            .collect();

        for y in 0..columns {
            for x in 0..rows {
                grid[y][x] += 1;
            }
        }
        //print_grid(&grid);

        let all_less_nine = grid.iter().map(|row| row.iter().all(|&v| v <= 9)).all(|v| v);
        if all_less_nine {
            print_grid(&grid);
            continue;
        }

        loop {
            let mut updated = false;
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    updated = updated || flash(&mut grid, &mut flashed, (x, y));
                }
            }
            if !updated {
                break;
            }
            //print_grid(&grid);
        }

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if flashed[y][x] {
                    grid[y][x] = 0;
                } else {
                    continue;
                }
            } 
        }

        print_grid(&grid);
        //print_grid(&flashed);
        //print_grid(&flashed);
        println!("");
    }
}
