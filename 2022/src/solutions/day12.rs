use aoc::Grid;
use pathfinding::prelude::bfs;

fn parse(input: &str) -> (Grid<char>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let col = line
            .chars()
            .enumerate()
            .filter(|(_, val)| *val == 'S')
            .map(|(idx, _)| idx)
            .nth(0);
        if let Some(x) = col {
            start = (x, y);
            break;
        }
    }
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let col = line
            .chars()
            .enumerate()
            .filter(|(_, val)| *val == 'E')
            .map(|(idx, _)| idx)
            .nth(0);
        if let Some(x) = col {
            end = (x, y);
            break;
        }
    }
    (
        Grid::new(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            if c == 'S' {
                                'a'
                            } else if c == 'E' {
                                'z'
                            } else {
                                c
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ),
        start,
        end,
    )
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, grid: &Grid<char>) -> Vec<Pos> {
        let &Pos(x, y) = self;

        let current = grid.at(x, y);
        let adj = grid
            .adjacent_positions(x, y)
            .into_iter()
            .filter(|(xx, yy)| {
                let curr = *current as i32;
                let p = *grid.at(*xx, *yy) as i32;

                p <= curr + 1
            })
            .map(|(xx, yy)| Pos(xx, yy))
            .collect::<Vec<_>>();
        adj
    }
}

fn shortest_path(
    (grid, start, end): (Grid<char>, (usize, usize), (usize, usize)),
) -> Option<usize> {
    let goal = Pos(end.0, end.1);
    let result = bfs(
        &Pos(start.0, start.1),
        |p| p.successors(&grid),
        |p| *p == goal,
    );
    if let Some(path) = result {
        Some(path.len() - 1)
    } else {
        None
    }
}

fn part_a((grid, start, end): (Grid<char>, (usize, usize), (usize, usize))) -> usize {
    shortest_path((grid, start, end)).unwrap()
}

fn part_b((grid, start, end): (Grid<char>, (usize, usize), (usize, usize))) -> usize {
    let (columns, rows) = grid.bounds();
    (0..rows)
        .flat_map(|y| {
            (0..columns)
                .map(|x| ((x, y), grid.at(x, y)))
                .collect::<Vec<_>>()
        })
        .filter(|(_, val)| *val == &'a')
        .map(|(coords, _)| coords)
        .map(|coords| shortest_path((grid.clone(), coords, end)))
        .filter(|res| res.is_some())
        .min()
        .unwrap()
        .unwrap()
}

pub fn day12(input: &str) -> (String, String) {
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
        let input = read_file("examples", 12);
        assert_eq!(part_a(parse(&input.unwrap())), 31);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 12);
        assert_eq!(part_b(parse(&input.unwrap())), 29);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 12);
        assert_eq!(part_a(parse(&input.unwrap())), 391);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 12);
        assert_eq!(part_b(parse(&input.unwrap())), 386);
    }
}
