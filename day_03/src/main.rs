use std::fs;

fn main() {
    let filename = "test.txt";
    //let filename = "input.txt";

    let str_lines = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<Vec<u32>> = str_lines
        .lines()
        .into_iter()
        .map(|l| {
            l.chars()
                .filter_map(|n| n.to_string().parse().ok())
                .collect()
        })
        .collect();

    // Part A
    let rows = lines.len();
    let columns = lines[0].len();

    let values = (0..columns)
        .into_iter()
        .map(|c| (0..rows).into_iter().map(|r| lines[r][c]).sum());


    println!("{:?}", values.clone().collect::<Vec<u32>>());

    let binary_values = values
        .clone()
        .map(|s: u32| (s > (rows as u32) / 2) as u32)
        .collect::<Vec<u32>>();

    let b_values = values
        .clone()
        .map(|ones: u32| if ones < (rows as u32) - ones { 1 } else { if (rows as u32) - ones <= ones { 0 } else { 1 } })
        .collect::<Vec<u32>>();

    println!("{}", rows);

    println!("{:?}", b_values);

    let gamma = binary_values
        .iter()
        .fold(0, |accum, &item| (accum << 1) + item as u32);

    let epsilon = binary_values
        .iter()
        .fold(0, |accum, &item| (accum << 1) + (item != 1 as u32) as u32);

    println!("Part A: {}", gamma * epsilon);

    // Part B
    let binary_numbers: Vec<u32> = str_lines
        .lines()
        .clone()
        .filter_map(|num| u32::from_str_radix(num, 2).ok())
        .collect();

    let mut accum = 0;
    let mut i = 0;

    for v in values {
        println!("{}", v);
        accum = accum*2 + v;
    
        let remaining_nums = binary_numbers.iter().filter(|&num| accum == (num >> i));
        println!("{}", remaining_nums.collect::<Vec<&u32>>().len());
        i += 1;
    }

    println!("Part B");

    let oxygen_generator_rating = binary_numbers
        .iter()
        .filter(|&num| (num >> 1) == (gamma >> 1))
        .max()
        .unwrap();
    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("{:?}", binary_numbers);

    let binary_numbers: Vec<u32> = str_lines
        .lines()
        .filter_map(|num| u32::from_str_radix(num, 2).ok())
        .filter(|num| (num >> 1) == epsilon)
        .collect();

    println!("{:#b}", gamma);
    println!("{:#b}", epsilon);

    println!("{:?}", binary_numbers);
}
