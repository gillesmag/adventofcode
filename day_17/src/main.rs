use std::fs;

fn main() {
    let filename = "input.txt";
    //let filename = "test.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let line = file.lines().collect::<Vec<&str>>()[0];

    let parts: Vec<_> = line.split(" ").collect();
    let x_range: &str = parts[2].strip_suffix(",").unwrap().split("=").collect::<Vec<&str>>()[1];
    let y_range: &str = parts[3].split("=").collect::<Vec<&str>>()[1];

    let xs: Vec<_> = x_range.split("..").filter_map(|v| v.parse::<i32>().ok()).collect();
    let ys: Vec<_> = y_range.split("..").filter_map(|v| v.parse::<i32>().ok()).collect();

    let max_height = (ys[0]..=ys[1]).map(|y| (-y)*((-y)-1)/2).max().unwrap();
    println!("{:?}", max_height);

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
    println!("{:?}", candidates.len());
}
