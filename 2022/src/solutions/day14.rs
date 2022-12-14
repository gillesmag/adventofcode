use aoc::Grid;
use itertools::Itertools;
// use image::{ImageBuffer, Rgb};

type Pos = (usize, usize);

type Input = Vec<Vec<Pos>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let c = coord
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (c[0], c[1])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn bounds(input: Input) -> (Pos, Pos) {
    let points = input
        .iter()
        .flat_map(|path| path.iter().collect::<Vec<_>>());

    let xs = points.clone().map(|(x, _)| x);
    let ys = points.clone().map(|(_, y)| y);

    (
        (*xs.clone().min().unwrap(), *xs.max().unwrap()),
        (*ys.clone().min().unwrap(), *ys.max().unwrap()),
    )
}

fn part_a(input: Input) -> usize {
    let (x_bounds, y_bounds) = bounds(input.clone());

    let width = x_bounds.1 - x_bounds.0 + 1;
    let height = y_bounds.1 + 1;

    let mut points = vec![];

    for path in &input {
        for pair in path.iter().map(|t| *t).tuple_windows::<(Pos, Pos)>() {
            if pair.0 .0 == pair.1 .0 {
                let ys = vec![pair.0 .1, pair.1 .1];
                let (min, max) = (*ys.clone().iter().min().unwrap(), *ys.iter().max().unwrap());
                for i in min..=max {
                    points.push((pair.0 .0 - x_bounds.0, i));
                }
            } else if pair.0 .1 == pair.1 .1 {
                let xs = vec![pair.0 .0, pair.1 .0];
                let (min, max) = (*xs.clone().iter().min().unwrap(), *xs.iter().max().unwrap());
                for i in min..=max {
                    points.push((i - x_bounds.0, pair.0 .1));
                }
            }
        }
    }

    // set walls
    let g = vec![vec!['.'; width + 2]; height + 1];
    // let mut img = ImageBuffer::new(width as u32 + 2, height as u32);
    // for (_, _, pixel) in img.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([0xff, 0xff, 0xff]);
    // }
    let mut grid = Grid::new(g);
    for point in &points {
        grid.set(point.0 + 1, point.1, '#');
        // img.put_pixel(point.0 as u32 +1, point.1 as u32, Rgb([0u8, 0u8, 0u8]));
    }

    let start = (500 - x_bounds.0 + 1, 0);

    let mut counter = 0;
    'outer: loop {
        let mut sand = start;
        grid.set(sand.0, sand.1, '+');
        loop {
            if sand.0 == 0 || sand.0 == width || sand.1 == height {
                break 'outer;
            }
            grid.set(sand.0, sand.1, '.');
            let left = *grid.at(sand.0 - 1, sand.1 + 1);
            let below = *grid.at(sand.0, sand.1 + 1);
            let right = *grid.at(sand.0 + 1, sand.1 + 1);
            if below == '.' {
                // below is free so go there
                grid.set(sand.0, sand.1 + 1, '+');
                sand = (sand.0, sand.1 + 1);
                continue;
            }
            // below is occupied so try left
            if left == '.' {
                grid.set(sand.0 - 1, sand.1 + 1, '+');
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }

            // left is occupied so try right
            if right == '.' {
                grid.set(sand.0 + 1, sand.1 + 1, '+');
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }

            // right is also blocked, so just pile up
            grid.set(sand.0, sand.1, 'o');
            counter += 1;
            // img.put_pixel(sand.0 as u32, sand.1 as u32, Rgb([0x96u8, 0x4bu8, 0u8]));
            // img.save(format!("/Users/gm/Downloads/day14/image_{:04}.gif", counter)).unwrap();
            break;
        }
    }

    counter
}

fn part_b(input: Input) -> usize {
    let (x_bounds, y_bounds) = bounds(input.clone());

    let width = 6 * (x_bounds.1 - x_bounds.0 + 1);
    let height = y_bounds.1 + 1;

    let mut points = vec![];

    for path in &input {
        for pair in path.iter().map(|t| *t).tuple_windows::<(Pos, Pos)>() {
            if pair.0 .0 == pair.1 .0 {
                let ys = vec![pair.0 .1, pair.1 .1];
                let (min, max) = (*ys.clone().iter().min().unwrap(), *ys.iter().max().unwrap());
                for i in min..=max {
                    points.push((
                        pair.0 .0 - x_bounds.0 + width / 2 - (x_bounds.1 - x_bounds.0) / 2,
                        i,
                    ));
                }
            } else if pair.0 .1 == pair.1 .1 {
                let xs = vec![pair.0 .0, pair.1 .0];
                let (min, max) = (*xs.clone().iter().min().unwrap(), *xs.iter().max().unwrap());
                for i in min..=max {
                    points.push((
                        i - x_bounds.0 + width / 2 - (x_bounds.1 - x_bounds.0) / 2,
                        pair.0 .1,
                    ));
                }
            }
        }
    }

    // set walls
    let g = vec![vec!['.'; width + 2]; height + 2];
    // let mut img = ImageBuffer::new(width as u32 + 2, height as u32 + 3);
    // for (_, _, pixel) in img.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([0xff, 0xff, 0xff]);
    // }

    let mut grid = Grid::new(g);
    for point in &points {
        grid.set(point.0 + 1, point.1, '#');
        // img.put_pixel(point.0 as u32 +1, point.1 as u32, Rgb([0u8, 0u8, 0u8]));
    }
    for x in 0..width + 2 {
        grid.set(x, y_bounds.1 + 2, '#');
        // img.put_pixel(x as u32, y_bounds.1 as u32 + 2, Rgb([0u8, 0u8, 0u8]));
    }

    let start = (
        500 - x_bounds.0 + 1 + width / 2 - (x_bounds.1 - x_bounds.0) / 2,
        0,
    );

    let mut counter = 0;
    'outer: loop {
        let mut sand = start;
        loop {
            if *grid.at(start.0, start.1) == 'o' {
                break 'outer;
            }
            grid.set(sand.0, sand.1, '+');
            let left = *grid.at(sand.0 - 1, sand.1 + 1);
            let below = *grid.at(sand.0, sand.1 + 1);
            let right = *grid.at(sand.0 + 1, sand.1 + 1);
            if below == '.' {
                // below is free so go there
                grid.set(sand.0, sand.1, '.');
                grid.set(sand.0, sand.1 + 1, '+');
                sand = (sand.0, sand.1 + 1);
                continue;
            }
            // below is occupied so try left
            if left == '.' {
                grid.set(sand.0, sand.1, '.');
                grid.set(sand.0 - 1, sand.1 + 1, '+');
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }

            // left is occupied so try right
            if right == '.' {
                grid.set(sand.0, sand.1, '.');
                grid.set(sand.0 + 1, sand.1 + 1, '+');
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }

            // right is also blocked, so just pile up
            grid.set(sand.0, sand.1, 'o');
            counter += 1;
            // img.put_pixel(sand.0 as u32, sand.1 as u32, Rgb([0x96u8, 0x4bu8, 0u8]));
            // img.save(format!("/Users/gm/Downloads/day14_2/image_{:06}.bmp", counter)).unwrap();
            break;
        }
    }

    counter
}

pub fn day14(input: &str) -> (String, String) {
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
        let input = read_file("examples", 14);
        assert_eq!(part_a(parse(&input.unwrap())), 24);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 14);
        assert_eq!(part_b(parse(&input.unwrap())), 93);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 14);
        assert_eq!(part_a(parse(&input.unwrap())), 1016);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 14);
        assert_eq!(part_b(parse(&input.unwrap())), 25402);
    }
}
