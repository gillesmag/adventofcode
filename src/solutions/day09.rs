use aoc::Grid;

fn compute_low_points(grid: &Grid) -> Vec<(usize, usize)> {
    let bounds = grid.bounds();
    let mut low_points = vec![];

    for (x, y) in (0..bounds.1).flat_map(|y| (0..bounds.0).map(move |x| (x, y))) {
        let value = grid.at(x, y);
        let adj = grid.adjacent_positions(x, y);

        let min_val = adj.into_iter().map(|(x, y)| grid.at(x, y)).min().unwrap();
        if value < min_val {
            low_points.push((x, y));
        }
    }

    low_points
}

fn part_a(grid: &Grid) -> u32 {
    compute_low_points(grid)
        .into_iter()
        .map(|(x, y)| grid.at(x, y) + 1)
        .sum()
}

fn compute_basin_size(grid: &Grid, point: (usize, usize)) -> usize {
    let (columns, rows) = grid.bounds();
    let mut visited = (0..rows)
        .into_iter()
        .map(|_| {
            (0..columns)
                .into_iter()
                .map(|_| false)
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    visited[point.1][point.0] = true;

    let mut positions_queue: Vec<(usize, usize)> = vec![];
    positions_queue.append(&mut grid.adjacent_positions(point.0, point.1).clone());

    let mut size = 1usize;
    while positions_queue.len() > 0 {
        let mut remove_positions: Vec<(usize, usize)> = vec![];
        let mut process_queue: Vec<(usize, usize)> = vec![];
        for v in &positions_queue {
            let value = grid.at(v.0, v.1);
            if visited[v.1][v.0] || value == 9 {
                remove_positions.push(*v);
                continue;
            }

            size += 1;
            visited[v.1][v.0] = true;
            process_queue.append(&mut grid.adjacent_positions(v.0, v.1).clone());
        }

        for v in remove_positions {
            positions_queue.retain(|&x| x != v);
        }
        positions_queue.append(&mut process_queue);
    }
    size
}

fn part_b(grid: &Grid) -> usize {
    // 1. get adjacent positions for low points
    // 2. check if their value is greater than current low point
    // 3. if so (and it is not 9) add add 1 and add them as a point to explore as well
    //compute_low_points(grid)
    let mut basin_sizes: Vec<usize> = vec![];

    for low_point in compute_low_points(grid) {
        basin_sizes.push(compute_basin_size(grid, low_point));
    }
    basin_sizes.sort();
    basin_sizes.into_iter().rev().take(3).product::<usize>()
}

fn parse(input: &str) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|v| String::from(v).parse().ok())
                    .collect::<Vec<u32>>()
            })
            .collect(),
    )
}

pub fn day09(input: &str) -> (String, String) {
    let grid = parse(input);
    (part_a(&grid).to_string(), part_b(&grid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 9);
        let grid = parse(&input);
        assert_eq!(part_a(&grid), 15);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 9);
        let grid = parse(&input);
        assert_eq!(part_b(&grid), 1134);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 9);
        let grid = parse(&input);
        assert_eq!(part_a(&grid), 502);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 9);
        let grid = parse(&input);
        assert_eq!(part_b(&grid), 1330560);
    }
}
