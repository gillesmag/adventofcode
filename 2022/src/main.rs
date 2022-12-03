use aoc::read_file;
use std::env;

mod solutions;

use solutions::day01::day01;
use solutions::day02::day02;
use solutions::day03::day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();
    let input = read_file("inputs", day);

    let (part_a, part_b) = match day {
        1 => day01(&input),
        2 => day02(&input),
        3 => day03(&input),
        _ => ("".to_string(), "".to_string()),
    };
    println!("Part A: {}", part_a);
    println!("Part B: {}", part_b);
}
