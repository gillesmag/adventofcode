use std::fs;

fn main() {
    let filename = "test.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let h: Vec<&str> = file.split("\n\n").collect();
    let algorithm = h[0].chars().map(|c| match c {
        '.' => false,
        '#' => true,
        _ => unreachable!("Invalid character in input")
    }).collect::<Vec<bool>>();
    let input_image: Vec<Vec<bool>> = h[1].lines().map(|line| line.chars().map(|c| match c {
        '.' => false,
        '#' => true,
        _ => unreachable!("Invalid character in input")
    }).collect::<Vec<bool>>()).collect();

    println!("{:?}", algorithm);
    println!("{:?}", input_image);
}
