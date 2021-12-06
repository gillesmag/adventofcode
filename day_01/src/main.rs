use std::fs;

fn part_a(nums: Vec<u32>) -> usize {
    nums.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_b(nums: Vec<u32>) -> usize {
    let depth_triples = nums.windows(3).map(|w| w.iter().sum()).collect();
    part_a(depth_triples)
}

fn main() {
    let filename = "input.txt";
    //let filename = "test.txt";
    println!("Reading {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let depths: Vec<u32> = contents.lines().filter_map(|depth| depth.parse().ok()).collect();
    println!("Part A: {}", part_a(depths.clone()));
    println!("Part B: {}", part_b(depths.clone()));
}
