use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::grid::Grid;

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
        _ => None,
    }
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter_map(|v| String::from(v).parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn part_a(weights: &Vec<Vec<usize>>) -> usize {
    total_risk(&weights).unwrap()
}

fn part_b(weights: &Vec<Vec<usize>>) -> usize {
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

    total_risk(&new_weights).unwrap()
}

pub fn day15(input: &str) -> (String, String) {
    //let filename = "test.txt";
    let weights = parse(input);
    (part_a(&weights).to_string(), part_b(&weights).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 15);
        let weights = parse(&input);
        assert_eq!(part_a(&weights), 40);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 15);
        let weights = parse(&input);
        assert_eq!(part_b(&weights), 315);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 15);
        let weights = parse(&input);
        assert_eq!(part_a(&weights), 714);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 15);
        let weights = parse(&input);
        assert_eq!(part_b(&weights), 2948);
    }
}
