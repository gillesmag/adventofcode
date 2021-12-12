use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    //let filename = "test_small.txt";
    //let filename = "test_medium.txt";
    //let filename = "test_large.txt";
    let filename = "input.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");

    let edges: Vec<Vec<&str>> = file
        .lines()
        .map(|line| line.split("-").collect::<Vec<&str>>())
        .collect();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut vertices: HashSet<&str> = HashSet::new();

    for edge in &edges {
        let (start, end) = (edge[0], edge[1]);
        //println!("{} {}", start, end);

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

    let mut paths: Vec<Vec<&str>> = vec![];

    let mut prefixes = vec![vec!["start"]];
    let mut new_prefixes: Vec<Vec<&str>> = vec![];

    loop {
        for prefix in &prefixes {
            let l = prefix.last().unwrap();
            if let Some(next)  = graph.get(l) {
                //println!("{:?}", next);
                for n in next {
                    // add one where one is unique and the other with the second occurence
                    //println!("{:?}", prefix);
                    if prefix.contains(&n) && n.chars().all(|v| v.is_ascii_lowercase()) {
                    //if prefix.contains(&n) && n.chars().all(|v| v.is_ascii_lowercase()) {
                        continue;
                    }
                    let mut new = prefix.clone();
                    new.push(n);
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
                paths.push(prefix.clone());
            }
        }

        prefixes = new_prefixes.clone();
        new_prefixes.clear();
        //println!("prefixes: {:?}", prefixes);
        //println!("new prefixes: {:?}", new_prefixes);

        if prefixes.len() == 0 {
            break;
        }
    }


    //for comps in &paths {
    //    for (idx, c) in comps.into_iter().enumerate() {
    //        print!("{}", c);
    //        if idx != comps.len()-1 {
    //            print!(",");
    //        }
    //    }
    //    println!("");
    //}

    println!("{:?}", paths.len());

    //println!("");
    //println!("");
    //println!("Vertices: {:?}", vertices);
    //println!("{:?}", graph);
    //println!("{:?}", edges);
}
