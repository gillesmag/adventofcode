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

    match day {
        1 => {
            let (part_a, part_b) = day01(&input);
            println!("Part A: {}", part_a);
            println!("Part B: {}", part_b);
            ()
        }
        2 => {
            let (part_a, part_b) = day02(&input);
            println!("Part A: {}", part_a);
            println!("Part B: {}", part_b);
            ()
        }
        3 => {
            let (part_a, part_b) = day03(&input);
            println!("Part A: {}", part_a);
            println!("Part B: {}", part_b);
            ()
        }
        4 => {
            let (part_a, part_b) = day04(&input);
            println!("Part A: {}", part_a);
            println!("Part B: {}", part_b);
            ()
        }
        5 => {
            let (part_a, part_b) = day05(&input);
            println!("Part A: {}", part_a);
            println!("Part B: {}", part_b);
            ()
        }
        6 => day06(),
        7 => day07(),
        8 => day08(),
        9 => day09(),
        10 => day10(),
        //11 => day11(),
        12 => day12(),
        13 => day13(),
        14 => day14(),
        15 => day15(),
        16 => day16(),
        17 => day17(),
        //18 => day18(),
        //19 => day19(),
        //20 => day20(),
        21 => day21(),
        _ => println!("day not solved: {}", day),
    }
}
