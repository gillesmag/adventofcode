use aoc::Grid;

fn parse(input: &str) -> Grid<usize> {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|v| v as usize - '0' as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

fn part_a(g: Grid<usize>) -> usize {
    let bounds = g.bounds();
    let mut inner_visible = 0;
    for y in 1..bounds.1 - 1 {
        for x in 1..bounds.0 - 1 {
            let target = *g.at(x, y);
            let left = (0..=x).map(|xx| *g.at(xx, y)).collect::<Vec<_>>();
            let right = (x..bounds.0)
                .rev()
                .map(|xx| *g.at(xx, y))
                .collect::<Vec<_>>();
            let top = (0..=y).map(|yy| *g.at(x, yy)).collect::<Vec<_>>();
            let bottom = (y..bounds.1)
                .rev()
                .map(|yy| *g.at(x, yy))
                .collect::<Vec<_>>();

            let sides = vec![left, right, top, bottom];

            let result = sides
                .iter()
                .map(|side| {
                    let m = side.iter().max().unwrap();
                    side.iter().filter(|val| *val == m).count() == 1 && *m == target
                })
                .any(|v| v);

            if result {
                inner_visible += 1;
            }
        }
    }
    bounds.0 * 2 + (bounds.1 - 2) * 2 + inner_visible
}

fn part_b(g: Grid<usize>) -> usize {
    let mut max_score = 0;
    let bounds = g.bounds();
    for y in 1..bounds.1 - 1 {
        for x in 1..bounds.0 - 1 {
            let target = *g.at(x, y);
            let left = (0..x).rev().map(|xx| *g.at(xx, y)).collect::<Vec<_>>();
            let right = (x + 1..bounds.0).map(|xx| *g.at(xx, y)).collect::<Vec<_>>();
            let top = (0..y).rev().map(|yy| *g.at(x, yy)).collect::<Vec<_>>();
            let bottom = (y + 1..bounds.1).map(|yy| *g.at(x, yy)).collect::<Vec<_>>();

            let sides = vec![left, right, top, bottom];

            let result = sides
                .iter()
                .map(|side| {
                    let mut accum = vec![];
                    for val in side {
                        accum.push(*val);
                        if val >= &target {
                            break;
                        }
                    }
                    accum.len()
                })
                .product();
            if result > max_score {
                max_score = result;
            }
        }
    }
    max_score
}

pub fn day08(input: &str) -> (String, String) {
    (
        part_a(parse(&input)).to_string(),
        part_b(parse(&input)).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 8);
        assert_eq!(part_a(parse(&input.unwrap())), 21);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 8);
        assert_eq!(part_b(parse(&input.unwrap())), 8);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 8);
        assert_eq!(part_a(parse(&input.unwrap())), 1825);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 8);
        assert_eq!(part_b(parse(&input.unwrap())), 235200);
    }
}
