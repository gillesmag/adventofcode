use aoc::read_file;
use std::env;

mod solutions;

use solutions::day01::day01;
use solutions::day02::day02;
use solutions::day03::day03;
use solutions::day04::day04;
use solutions::day05::day05;

fn main() -> Result<(), &'static str> {
    let day: u8 = match env::args().nth(1) {
        Some(num) => match num.parse() {
            Ok(n) => n,
            Err(_) => return Err("Invalid number"),
        },
        None => return Err("Missing argument: day"),
    };

    let input = match read_file("inputs", day) {
        Ok(s) => s,
        Err(_) => return Err("could not open input file"),
    };

    let (part_a, part_b) = match day {
        1 => day01(&input),
        2 => day02(&input),
        3 => day03(&input),
        4 => day04(&input),
        5 => day05(&input),
        _ => return Err("Unknown day"),
    };
    println!("Part A: {}", part_a);
    println!("Part B: {}", part_b);

    Ok(())
}
