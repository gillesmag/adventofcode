use std::collections::HashMap;

type Coordinate = (u32, u32);

fn gen_positions(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    if start.0 == end.0 && start.1 != end.1 {
        let (min, max) = if start.1 < end.1 {
            (start.1, end.1)
        } else {
            (end.1, start.1)
        };
        return (min..=max)
            .into_iter()
            .map(|comp| (start.0, comp))
            .collect();
    } else if start.1 == end.1 && start.0 != end.0 {
        let (min, max) = if start.0 < end.0 {
            (start.0, end.0)
        } else {
            (end.0, start.0)
        };
        return (min..=max)
            .into_iter()
            .map(|comp| (comp, start.1))
            .collect();
    } else {
        let r1: Vec<u32> = if start.0 < end.0 {
            (start.0..=end.0).into_iter().collect()
        } else {
            (end.0..=start.0).rev().into_iter().collect()
        };
        let r2: Vec<u32> = if start.1 < end.1 {
            (start.1..=end.1).into_iter().collect()
        } else {
            (end.1..=start.1).rev().into_iter().collect()
        };
        return r1.into_iter().zip(r2.into_iter()).collect();
    }
}

fn parse(input: &str) -> Vec<(Coordinate, Coordinate)> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let coordinates = line
                .split(" -> ")
                .map(|coords| {
                    let components: Vec<u32> = coords
                        .split(",")
                        .filter_map(|component| component.parse().ok())
                        .collect();
                    (components[0], components[1])
                })
                .collect::<Vec<(u32, u32)>>();
            (coordinates[0], coordinates[1])
        })
        .collect()
}

fn compute_overlap(coordinates: &Vec<(Coordinate, Coordinate)>) -> usize {
    let mut seen_coords: HashMap<Coordinate, usize> = HashMap::new();

    for pair in coordinates {
        let (start, end) = (pair.0, pair.1);
        let positions = gen_positions(start, end);
        for position in positions {
            match seen_coords.get(&position) {
                Some(count) => seen_coords.insert(position, count + 1),
                None => seen_coords.insert(position, 1),
            };
        }
    }

    seen_coords.values().filter(|&&val| val > 1).count()
}

fn part_a(coordinates: &Vec<(Coordinate, Coordinate)>) -> usize {
    let simple_coords = coordinates
        .iter()
        .map(|v| v.clone())
        .filter(|(start, end)| {
            (start.0 == end.0 && start.1 != end.1) || (start.0 != end.0 && start.1 == end.1)
        })
        .collect::<Vec<(Coordinate, Coordinate)>>();
    compute_overlap(&simple_coords)
}

fn part_b(coordinates: &Vec<(Coordinate, Coordinate)>) -> usize {
    compute_overlap(coordinates)
}

pub fn day05(input: &str) -> (String, String) {
    let coordinates = parse(&input);
    (
        part_a(&coordinates).to_string(),
        part_b(&coordinates).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 5);
        let coordinates = parse(&input);
        assert_eq!(part_a(&coordinates), 5);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 5);
        let coordinates = parse(&input);
        assert_eq!(part_b(&coordinates), 12);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 5);
        let coordinates = parse(&input);
        assert_eq!(part_a(&coordinates), 8622);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 5);
        let coordinates = parse(&input);
        assert_eq!(part_b(&coordinates), 22037);
    }
}
