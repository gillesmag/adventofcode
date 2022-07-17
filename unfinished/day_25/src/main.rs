use std::fs;

fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) {
    println!();
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

fn main() {
    //let filename = "test.txt";
    //let filename = "test2.txt";
    //let filename = "test3.txt";
    //let filename = "test4.txt";
    //let filename = "test5.txt";
    let filename = "test6.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let mut grid = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    //print_grid(&grid);

    //let mut empty = grid
    //    .iter()
    //    .map(|row| {
    //        row.into_iter()
    //            .map(|cell| *cell == '.')
    //            .collect::<Vec<bool>>()
    //    })
    //    .collect::<Vec<Vec<bool>>>();
    //println!("{:?}", empty);

    for _round in 0..4 {
        print_grid(&grid);
        let mut empty = grid
            .iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| *cell == '.')
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        let mut new_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
        for (rid, row) in grid.iter().enumerate() {
            for (cid, cell) in row.iter().enumerate() {
                if *cell == '>' {
                    let (r, nc) = (rid, (cid + 1) % grid[0].len());
                    if empty[r][nc] {
                        new_grid[r][nc] = '>';
                        empty[r][nc] = false;
                        empty[r][cid] = true;
                    } else {
                        new_grid[r][cid] = '>';
                    }
                }
            }
        }

        for (rid, row) in grid.iter().enumerate() {
            for (cid, cell) in row.iter().enumerate() {
                if *cell == 'v' {
                    let (nr, c) = ((rid + 1) % grid.len(), cid);
                    if empty[nr][c] {
                        new_grid[nr][c] = 'v';
                        empty[nr][c] = false;
                        empty[rid][c] = true;
                    } else {
                        new_grid[rid][c] = 'v';
                    }
                }
            }
        }

        grid = new_grid;
    }

    //for round in 0..2 {
    //    let mut new_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    //    let mut changed = false;
    //    for (rid, row) in grid.iter().enumerate() {
    //        for (cid, col) in row.iter().enumerate() {
    //            match col {
    //                '>' => {
    //                    let (r, nc) = (rid, (cid + 1) % grid[0].len());
    //                    if grid[r][nc] == '.' && empty[r][cid] {
    //                        new_grid[r][cid] = '.';
    //                        empty[r][cid] = true;
    //                        new_grid[r][nc] = '>';
    //                        empty[r][nc] = false;
    //                        changed = true;
    //                    } else {
    //                        new_grid[r][cid] = '>';
    //                        empty[r][cid] = false;
    //                    }
    //                }
    //                'v' => {
    //                    new_grid[rid][cid] = 'v';
    //                    empty[rid][cid] = false;
    //                }
    //                '.' => {
    //                    if empty[rid][cid] {
    //                        new_grid[rid][cid] = '.';
    //                    }
    //                }
    //                _ => {}
    //            }
    //        }
    //    }
    //    //print_grid(&new_grid);
    //    //print_grid(&empty);

    //    for (rid, row) in grid.iter().enumerate() {
    //        for (cid, col) in row.iter().enumerate() {
    //            match col {
    //                'v' => {
    //                    let (nr, c) = ((rid + 1) % grid.len(), cid);
    //                    if new_grid[nr][c] == '.' && empty[nr][c] {
    //                        new_grid[rid][c] = '.';
    //                        empty[rid][c] = true;
    //                        new_grid[nr][c] = 'v';
    //                        empty[nr][c] = false;
    //                        changed = true;
    //                    } else {
    //                        new_grid[rid][c] = 'v';
    //                        empty[rid][c] = false;
    //                    }
    //                }
    //                _ => {}
    //            }
    //        }
    //    }

    //    grid = new_grid;

    //    println!("{}", round);
    //    print_grid(&grid);
    //    if !changed {
    //        println!("{}", round);
    //        break;
    //    }
    //}

    //println!();
    //print_grid(&grid);
    //println!("Old grid: {:?}", lines);
    //println!("New grid: {:?}", new_grid);
}
