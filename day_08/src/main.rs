use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn parse_segment(positions: &Vec<usize>) -> Option<usize> {
    let segment_positions: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1
        vec![0, 2, 3, 4, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6],    // 9
    ];
    for (num, segment_configuration) in segment_positions.into_iter().enumerate() {
        if segment_configuration == *positions {
            return Some(num);
        }
    }
    None
}

fn main() {
    //let filename = "test1.txt";
    //let filename = "test2.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");

    let input = file
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|part| part.split(" ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect::<Vec<_>>();

    let mut counter = 0;

    for line in &input {
        let (patterns, output) = (&line[0], &line[1]);
        let lengths = output.into_iter().map(|v| v.len()).collect::<Vec<usize>>();
        //println!("{:?}: {:?}", output, lengths);

        for length in lengths {
            //println!("{}", length);
            if [2, 4, 3, 7].contains(&length) {
                counter += 1;
            }
        }
    }
    //println!("{}", counter);

    let perms = ('a'..='g')
        .into_iter()
        .permutations(7)
        .collect::<Vec<Vec<char>>>();
    //println!("{:?}", perms);

    let digit_mapping: HashMap<&str, usize> = HashMap::new();

    let mut total = 0;

    for line in &input {
        let (patterns, output) = (&line[0], &line[1]);
        let lengths = output.into_iter().map(|v| v.len()).collect::<Vec<usize>>();
        //println!("{:?}: {:?}", output, lengths);
        for perm in &perms {
            //let perm = vec!['d', 'e', 'a', 'f', 'g', 'b', 'c'];
            //let perm = &perms[0];
            //println!("{:?}", perm);
            let mut values: Vec<Option<usize>> = vec![];
            for pattern in patterns {
                let mut positions = pattern
                    .chars()
                    .filter_map(|val| perm.iter().position(|&x| val == x))
                    .collect::<Vec<usize>>();
                positions.sort();
                let segment_value = parse_segment(&positions);
                values.push(segment_value);
                //println!("{:?}: {:?}", pattern, positions);
            }
            //println!("{:?}", values);
            //println!("{:?}", values.iter().all(|v| v.is_some()));
            if values.iter().all(|v| v.is_some()) {
                //println!("perm: {:?}", perm);
                //println!("values: {:?}", values);
                //println!("pattern: {:?}", patterns);
                let sorted_patterns = patterns
                    .into_iter()
                    .map(|val| {
                        let mut vals = val.chars().collect::<Vec<char>>();
                        vals.sort();
                        vals
                    })
                    .collect::<Vec<Vec<char>>>();
                //println!("{:?}", sorted_patterns);
                let output_val = output
                    .into_iter()
                    .map(|val| {
                        let mut vals = val.chars().collect::<Vec<char>>();
                        vals.sort();
                        sorted_patterns.iter().position(|v| vals == *v)
                        //vals
                    })
                    .filter_map(|v| v)
                    .filter_map(|idx| values[idx])
                    .fold(0, |acc, val| acc * 10 + val);
                    //.collect::<Vec<usize>>();
                total += output_val;
                println!("output: {:?}", output_val);
                break;
            }
        }
    }
    println!("Total: {:?}", total);
}
