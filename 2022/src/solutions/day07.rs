use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum FSEntryType {
    File,
    Directory,
}

#[derive(Debug, PartialEq, Clone)]
struct FSEntry {
    fs_type: FSEntryType,
    name: String,
    size: usize,
}

impl FSEntry {
    fn directory(name: String) -> Self {
        Self {
            fs_type: FSEntryType::Directory,
            name,
            size: 0,
        }
    }
}

#[derive(Debug)]
struct Node {
    idx: usize,
    val: FSEntry,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(idx: usize, val: FSEntry) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    arena: Vec<Node>,
}

impl FileSystem {
    fn new() -> Self {
        Self { arena: vec![] }
    }

    fn node(&mut self, val: FSEntry) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
}

fn parse(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let root = fs.node(FSEntry::directory("/".into()));
    let mut current_dir = root;
    let groups = input.split("$").skip(1);

    for group in groups {
        let lines = group.lines();
        let cli = lines
            .clone()
            .nth(0)
            .unwrap()
            .trim()
            .split(" ")
            .collect::<Vec<_>>();
        let command = cli[0];
        let output = lines.skip(1).collect::<Vec<_>>();

        if command == "cd" {
            let dir = cli[1];
            if dir == "/" {
                current_dir = root;
            } else if dir == ".." {
                current_dir = fs.arena[current_dir].parent.unwrap();
            } else {
                let child_dir = fs.arena[current_dir]
                    .children
                    .iter()
                    .filter(|&&child| fs.arena[child].val.name == dir)
                    .collect::<Vec<_>>();

                if child_dir.len() == 1 {
                    current_dir = *child_dir[0];
                } else {
                    panic!(
                        "unknown child {} in {}",
                        dir, fs.arena[current_dir].val.name
                    );
                }
            }
        } else if command == "ls" {
            for entry in output {
                let cols = entry.split(" ").collect_tuple::<(&str, &str)>().unwrap();

                let node = if cols.0 == "dir" {
                    let c = fs.node(FSEntry::directory(cols.1.to_string()));
                    fs.arena[c].parent = Some(current_dir);
                    c
                } else {
                    fs.node(FSEntry {
                        fs_type: FSEntryType::File,
                        name: cols.1.to_string(),
                        size: cols.0.parse::<usize>().unwrap(),
                    })
                };

                fs.arena[current_dir].children.push(node);
            }
        }
    }

    fs
}

fn get_size(fs: &FileSystem, node_id: usize) -> usize {
    let node = &fs.arena[node_id];
    if node.val.fs_type != FSEntryType::Directory {
        return node.val.size;
    }

    let mut size = 0;
    for child in &node.children {
        size += get_size(fs, *child);
    }

    size
}

fn part_a(input: &str) -> usize {
    let fs = parse(&input);
    fs.arena
        .iter()
        .filter(|node| node.val.fs_type == FSEntryType::Directory)
        .map(|dir| get_size(&fs, dir.idx))
        .filter(|size| *size < 100000)
        .sum::<usize>()
}

fn part_b(input: &str) -> usize {
    let fs = parse(&input);
    let used = 70000000 - get_size(&fs, 0);
    let needed = 30000000 - used;
    fs.arena
        .iter()
        .filter(|node| node.val.fs_type == FSEntryType::Directory)
        .map(|dir| get_size(&fs, dir.idx))
        .filter(|size| *size > needed)
        .min()
        .unwrap()
}

pub fn day07(input: &str) -> (String, String) {
    (part_a(&input).to_string(), part_b(&input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_filesystem() {
        let mut tree = FileSystem::new();
        let root = tree.node(FSEntry::directory("/".to_string()));
        let a = tree.node(FSEntry::directory("a".to_string()));
        let b = tree.node(FSEntry::directory("b".to_string()));

        tree.arena[root].children.push(a);
        tree.arena[a].parent = Some(root);
        tree.arena[b].parent = Some(a);

        assert_eq!(tree.arena[b].parent, Some(a));
    }

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 7);
        assert_eq!(part_a(&input.unwrap()), 95437);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 7);
        assert_eq!(part_b(&input.unwrap()), 24933642);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 7);
        assert_eq!(part_a(&input.unwrap()), 1644735);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 7);
        assert_eq!(part_b(&input.unwrap()), 1300850);
    }
}
