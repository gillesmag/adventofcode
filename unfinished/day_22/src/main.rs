use std::cmp::{max, min};
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cuboid {
    On,
    Off,
}

fn interval_intersection(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    //println!("{:?} {:?}", a, b);
    if b.0 > a.1 || a.0 > b.1 {
        None
    } else {
        Some((max(a.0, b.0), min(a.1, b.1)))
    }
}

fn compute_volume(xs: (i32, i32), ys: (i32, i32), zs: (i32, i32)) -> i32 {
    //println!("{}", ((xs.0.abs() - xs.1.abs()).abs() + 1));
    //println!("{}", ((ys.0.abs() - ys.1.abs()).abs() + 1));
    //println!("{}", ((zs.0.abs() - zs.1.abs()).abs()) + 1);
    println!("{:?} {:?} {:?}", xs, ys, zs);
    (((xs.0 - xs.1).abs() + 1)
        * ((ys.0 - ys.1).abs() + 1)
        * ((zs.0 - zs.1).abs() + 1)) as i32
}

fn main() {
    let filename = "test.txt";
    //let filename = "test2.txt";
    //let filename = "simple.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let lines = file.lines().collect::<Vec<&str>>();
    let instructions = lines
        .iter()
        .map(|v| {
            let vals = v.split(" ").collect::<Vec<&str>>();
            let cuboid = match vals[0] {
                "on" => Cuboid::On,
                "off" => Cuboid::Off,
                _ => unreachable!("Invalid state for cuboid"),
            };

            let ranges = vals[1]
                .split(",")
                .map(|axis| {
                    let range = axis.split("=").collect::<Vec<&str>>();
                    let range = range[1]
                        .split("..")
                        .filter_map(|val| val.parse::<i32>().ok())
                        .collect::<Vec<i32>>();
                    (range[0], range[1])
                })
                .collect::<Vec<(i32, i32)>>();
            (cuboid, ranges)
        })
        .collect::<Vec<_>>();

    println!("Instructions: {:?}", instructions);

    let mut active_cuboids: i32 = 0;
    let outside_range = |axis: (i32, i32)| axis.0 < -50 || axis.1 > 50;

    for (idx, (state, region)) in instructions.clone().into_iter().enumerate() {
        if outside_range(region[0]) || outside_range(region[1]) || outside_range(region[2]) {
            println!("outside");
            continue;
        }
        if state == Cuboid::On {
            println!("{:?} {:?} {:?}", region[0], region[1], region[2]);
            println!("+ first {}", compute_volume(region[0], region[1], region[2]));
            active_cuboids += compute_volume(region[0], region[1], region[2]);
        }
    }

    println!("subtract");

    for (idx, (state1, region1)) in instructions.clone().into_iter().enumerate() {
        if outside_range(region1[0]) || outside_range(region1[1]) || outside_range(region1[2]) {
            println!("outside");
            continue;
        }
        for (state2, region2) in instructions.clone().into_iter().skip(idx + 1) {
            if outside_range(region2[0]) || outside_range(region2[1]) || outside_range(region2[2]) {
                continue;
            }

            let xs = interval_intersection(region1[0], region2[0]);
            let ys = interval_intersection(region1[1], region2[1]);
            let zs = interval_intersection(region1[2], region2[2]);
            println!("{:?} {:?} {:?}", xs, ys, zs);
            // does the intersection exist?
            if [xs, ys, zs].into_iter().any(|v| v.is_none()) {
                continue;
            }
            let xs = xs.unwrap();
            let ys = ys.unwrap();
            let zs = zs.unwrap();

            // remove intersection if any is off
            if [state1, state2].into_iter().any(|v| v == Cuboid::Off) {
                println!("- {}", compute_volume(xs, ys, zs));
                active_cuboids -= compute_volume(xs, ys, zs);
            }
        }
    }

    println!("Active: {}", active_cuboids);
}
