use std::fs;

fn part_a(lines: Vec<(&str, i32)>) {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for line in lines {
        let (direction, amount) = line;

        if direction == "forward" {
            horizontal_pos += amount;
        } else if direction == "up" {
            vertical_pos -= amount;
        } else if direction == "down" {
            vertical_pos += amount;
        } else {
            panic!("Unsupported direction");
        }
    }

    println!("{}", horizontal_pos * vertical_pos);
}

fn part_b(lines: Vec<(&str, i32)>) {
    let mut aim = 0;
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for line in lines {
        let (direction, amount) = line;

        if direction == "forward" {
            horizontal_pos += amount;
            vertical_pos += aim * amount;
        } else if direction == "up" {
            aim -= amount;
        } else if direction == "down" {
            aim += amount;
        } else {
            panic!("Unsupported direction");
        }
    }

    println!("{}", horizontal_pos * vertical_pos);
}

fn main() {
    let filename = "input.txt";
    //let filename = "test.txt";
    let lines = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = lines
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| line.split(" ").collect())
        .map(|l: Vec<&str>| (l[0], l[1].parse::<i32>().expect("error parsing int")))
        .collect::<Vec<(&str, i32)>>();

    part_a(lines.clone());
    part_b(lines.clone());
}
