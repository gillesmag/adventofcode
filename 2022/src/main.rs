use aoc::read_file;
use std::env;

mod solutions;

use solutions::day01::day01;
use solutions::day02::day02;
use solutions::day03::day03;
use solutions::day04::day04;
use solutions::day05::day05;
use solutions::day06::day06;
use solutions::day07::day07;
use solutions::day08::day08;

use solutions::day10::day10;
use solutions::day11::day11;
// use solutions::day12::day12;

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
        6 => day06(&input),
        7 => day07(&input),
        8 => day08(&input),
        10 => day10(&input),
        11 => day11(&input),
        // 12 => day12(&input),
        _ => return Err("Unknown day"),
    };

    println!("Part A: {}", part_a);
    println!("Part B: {}", part_b);

    Ok(())
}
