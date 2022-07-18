use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

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

fn parse(input: &str) -> Graph {
    let edges: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split("-").collect::<Vec<&str>>())
        .collect();

    let mut graph = HashMap::new();

    for edge in &edges {
        let (start, end) = (edge[0], edge[1]);

        let start_edges = graph.entry(start).or_insert(vec![]);
        start_edges.push(end);
        let end_edges = graph.entry(end).or_insert(vec![]);
        end_edges.push(start);
    }

    // Graph preprocessing
    for val in graph.values_mut() {
        let mut v: Vec<&str> = val.clone().into_iter().filter(|&v| v != "start").collect();
        v.sort();
        *val = v;
    }
    graph.remove("end");
    graph
}

fn process_graph(graph: Graph, visit_twice: bool) -> usize {
    let mut prefixes = vec![vec!["start"]];
    let mut new_prefixes: Vec<Vec<&str>> = vec![];
    let mut counter = 0;

    loop {
        for prefix in &prefixes {
            let l = prefix.last().unwrap();
            if let Some(next) = graph.get(l) {
                for n in next {
                    let at_least_once = if visit_twice {
                        contains_twice(&prefix)
                    } else {
                        true
                    };
                    if (prefix.contains(&n) && n.chars().all(|v| v.is_ascii_lowercase()))
                        && at_least_once
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
    counter
}

fn part_a(graph: Graph) -> usize {
    process_graph(graph, false)
}

fn part_b(graph: Graph) -> usize {
    process_graph(graph, true)
}

pub fn day12(input: &str) -> (String, String) {
    //let filename = "test_small.txt";
    //let filename = "test_medium.txt";
    //let filename = "test_large.txt";
    let graph = parse(input);
    (part_a(graph.clone()).to_string(), part_b(graph).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 12);
        let graph = parse(&input);
        assert_eq!(part_a(graph), 226);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 12);
        let graph = parse(&input);
        assert_eq!(part_b(graph), 3509);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 12);
        let graph = parse(&input);
        assert_eq!(part_a(graph), 4241);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 12);
        let graph = parse(&input);
        assert_eq!(part_b(graph), 122134);
    }
}
