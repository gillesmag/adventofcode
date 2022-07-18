fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let line = input.lines().collect::<Vec<&str>>()[0];

    let parts: Vec<_> = line.split(" ").collect();
    let x_range: &str = parts[2]
        .strip_suffix(",")
        .unwrap()
        .split("=")
        .collect::<Vec<&str>>()[1];
    let y_range: &str = parts[3].split("=").collect::<Vec<&str>>()[1];

    let xs: Vec<_> = x_range
        .split("..")
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    let ys: Vec<_> = y_range
        .split("..")
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();

    (xs, ys)
}

fn part_a(ys: &Vec<i32>) -> i32 {
    (ys[0]..=ys[1])
        .map(|y| (-y) * ((-y) - 1) / 2)
        .max()
        .unwrap()
}

fn part_b(xs: &Vec<i32>, ys: &Vec<i32>) -> usize {
    let max_x = xs[1];
    let max_y = ys[0];

    let mut candidates: Vec<(i32, i32)> = vec![];

    for dx in 1..=max_x {
        for dy in max_y..=-max_y {
            let mut pos = (0, 0);

            let mut ddx = dx;
            let mut ddy = dy;
            while pos.0 <= max_x && pos.1 >= max_y {
                if xs[0] <= pos.0 && pos.0 <= xs[1] && pos.1 >= ys[0] && pos.1 <= ys[1] {
                    candidates.push((ddx, ddy));
                    break;
                }
                pos.0 += ddx;
                pos.1 += ddy;
                if ddx > 0 {
                    ddx -= 1;
                }
                ddy -= 1;
            }
        }
    }

    candidates.len()
}

pub fn day17(input: &str) -> (String, String) {
    //let filename = "test.txt";
    let (xs, ys) = parse(input);
    (part_a(&ys).to_string(), part_b(&xs, &ys).to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 17);
        let (_, ys) = parse(&input);
        assert_eq!(part_a(&ys), 45);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 17);
        let (xs, ys) = parse(&input);
        assert_eq!(part_b(&xs, &ys), 112);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 17);
        let (_, ys) = parse(&input);
        assert_eq!(part_a(&ys), 2278);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 17);
        let (xs, ys) = parse(&input);
        assert_eq!(part_b(&xs, &ys), 996);
    }
}
