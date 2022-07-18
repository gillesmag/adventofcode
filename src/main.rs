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
use solutions::day09::day09;
use solutions::day10::day10;
//use solutions::day11::day11;
use solutions::day12::day12;
use solutions::day13::day13;
use solutions::day14::day14;
use solutions::day15::day15;
use solutions::day16::day16;
use solutions::day17::day17;
//use solutions::day18::day18;
//use solutions::day19::day19;
//use solutions::day20::day20;
use solutions::day21::day21;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();
    let input = read_file("inputs", day);

    let (part_a, part_b) = match day {
        1 => day01(&input),
        2 => day02(&input),
        3 => day03(&input),
        4 => day04(&input),
        5 => day05(&input),
        6 => day06(&input),
        7 => day07(&input),
        8 => day08(&input),
        9 => day09(&input),
        10 => day10(&input),
        //11 => day11(&input),
        12 => day12(&input),
        13 => day13(&input),
        14 => day14(&input),
        15 => day15(&input),
        16 => day16(&input),
        17 => day17(&input),
        //18 => day18(&input),
        //19 => day19(&input),
        //20 => day20(&input),
        21 => day21(&input),
        _ => ("".to_string(), "".to_string()),
    };
    println!("Part A: {}", part_a);
    println!("Part B: {}", part_b);
}
