use std::collections::{HashMap, HashSet};
use std::fs;

fn contains_twice(vals: &Vec<&str>) -> bool {
    let mut map: HashMap<&str, usize> = HashMap::new();

    for v in vals {
        if v.chars().all(|v| v.is_ascii_uppercase()) {
            continue;
        }
        let e = map.entry(v).or_insert(0);
        *e += 1;
    }

    map.values().into_iter().any(|&v| v == 2)
}

pub fn day12() {
    //let filename = "test_small.txt";
    //let filename = "test_medium.txt";
    //let filename = "test_large.txt";
    let filename = "src/inputs/day12.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");

    let edges: Vec<Vec<&str>> = file
        .lines()
        .map(|line| line.split("-").collect::<Vec<&str>>())
        .collect();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut vertices: HashSet<&str> = HashSet::new();

    for edge in &edges {
        let (start, end) = (edge[0], edge[1]);

        let start_edges = graph.entry(start).or_insert(vec![]);
        start_edges.push(end);
        let end_edges = graph.entry(end).or_insert(vec![]);
        end_edges.push(start);

        vertices.insert(start);
        vertices.insert(end);
    }

    // Graph preprocessing
    for val in graph.values_mut() {
        let mut v: Vec<&str> = val.clone().into_iter().filter(|&v| v != "start").collect();
        v.sort();
        *val = v;
    }
    graph.remove("end");

    let mut visited: HashMap<&str, bool> = HashMap::new();
    for k in vertices.iter() {
        visited.entry(k).or_insert(false);
    }

    let mut prefixes = vec![vec!["start"]];
    let mut new_prefixes: Vec<Vec<&str>> = vec![];
    let mut counter = 0;

    loop {
        for prefix in &prefixes {
            let l = prefix.last().unwrap();
            if let Some(next) = graph.get(l) {
                for n in next {
                    if (prefix.contains(&n) && n.chars().all(|v| v.is_ascii_lowercase()))
                        && contains_twice(&prefix)
                    {
                        continue;
                    }
                    let mut new = prefix.clone();
                    new.push(&n);
                    new_prefixes.push(new);
                }
            }
        }

        for prefix in &new_prefixes {
            if prefix.len() == 0 {
                continue;
            }
            let p = prefix.last().unwrap();
            if *p == "end" {
                counter += 1;
            }
        }

        prefixes = new_prefixes.clone();
        new_prefixes.clear();

        if prefixes.len() == 0 {
            break;
        }
    }

    println!("{}", counter);
}
