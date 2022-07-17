fn part_a(nums: Vec<u32>) -> usize {
    nums.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_b(nums: Vec<u32>) -> usize {
    let depth_triples = nums.windows(3).map(|w| w.iter().sum()).collect();
    part_a(depth_triples)
}

fn to_depths(contents: &str) -> Vec<u32> {
    contents.lines().filter_map(|depth| depth.parse().ok()).collect::<Vec<u32>>()
}

pub fn day01(input: &str) -> (String, String) {
    let depths = to_depths(&input);
    (part_a(depths.clone()).to_string(), part_b(depths.clone()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_part_a() {
        let input = read_file("examples", 1);
        let depths = to_depths(&input);
        assert_eq!(part_a(depths), 7);
    }

    #[test]
    fn test_part_b() {
        let input = read_file("examples", 1);
        let depths = to_depths(&input);
        assert_eq!(part_b(depths), 5);
    }
}
