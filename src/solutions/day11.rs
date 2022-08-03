use aoc::Grid;

type Coordinate = (usize, usize);

fn parse(input: &str) -> Grid {
    Grid::with_diagonals(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|v| String::from(v).parse().ok())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    )
}

fn make_flashed(grid: &Grid) -> Vec<Vec<bool>> {
    let (columns, rows) = grid.bounds();

    (0..rows)
        .into_iter()
        .map(|_| {
            (0..columns)
                .into_iter()
                .map(|_| false)
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn part_a(grid: &mut Grid, steps: u32) -> u32 {
    let mut flash_count = 0;

    for _ in 1..=steps {
        let mut flashed: Vec<Vec<bool>> = make_flashed(&grid);

        step(grid, &mut flashed);

        // Count flashed
        let (columns, rows) = grid.bounds();
        for y in 0..rows {
            for x in 0..columns {
                if flashed[y][x] {
                    flash_count += 1;
                }
            }
        }
    }

    flash_count
}

fn part_b(grid: &mut Grid) -> u32 {
    let mut current_step = 1;
    loop {
        let mut flashed: Vec<Vec<bool>> = make_flashed(&grid);
        step(grid, &mut flashed);
        let all_flashed = flashed.iter().map(|row| row.iter().all(|&v| v)).all(|v| v);
        if all_flashed {
            return current_step;
        }
        current_step += 1;
    }
}

fn step(grid: &mut Grid, flashed: &mut Vec<Vec<bool>>) {
    let (columns, rows) = grid.bounds();
    let mut flash_queue: Vec<Coordinate> = vec![];

    // Increment all cells by 1
    for y in 0..columns {
        for x in 0..rows {
            grid.set(x, y, grid.at(x, y) + 1);
        }
    }

    for y in 0..rows {
        for x in 0..columns {
            if grid.at(x, y) <= 9 {
                continue;
            }

            // Update values above 9
            // set to zero and add all adjacent cells to queue to trigger flashing
            for adj in grid.adjacent_positions(x, y) {
                flash_queue.push(adj);
                grid.set(x, y, 0);
                flashed[y][x] = true;
            }
        }
    }

    loop {
        let mut new_flash_queue: Vec<Coordinate> = vec![];
        if flash_queue.len() == 0 {
            break;
        }

        for (cx, cy) in flash_queue.iter() {
            // only count flashes at most once
            if flashed[*cy][*cx] {
                continue;
            }
            grid.set(*cx, *cy, grid.at(*cx, *cy) + 1);
            if grid.at(*cx, *cy) > 9 {
                flashed[*cy][*cx] = true;
                grid.set(*cx, *cy, 0);
                // trigger adjacent neighbours
                new_flash_queue.append(&mut grid.adjacent_positions(*cx, *cy));
            }
        }

        std::mem::swap(&mut flash_queue, &mut new_flash_queue);
    }
}

pub fn day11(input: &str) -> (String, String) {
    let mut grid = parse(input);
    (part_a(&mut grid, 200).to_string(), "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_small() {
        let input = "11111
19991
19191
19991
11111";
        let mut grid = parse(&input);
        assert_eq!(part_a(&mut grid, 2), 9);
    }

    #[test]
    fn test_example_large() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut grid = parse(&input);
        assert_eq!(part_a(&mut grid, 10), 204);
        let mut grid = parse(&input);
        assert_eq!(part_a(&mut grid, 100), 1656);
        // Part B
        let mut grid = parse(&input);
        assert_eq!(part_b(&mut grid), 195);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 11);
        let mut grid = parse(&input);
        assert_eq!(part_a(&mut grid, 100), 1679);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 11);
        let mut grid = parse(&input);
        assert_eq!(part_b(&mut grid), 519);
    }
}
