use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

type ItemPairs = Vec<(Value, Value)>;

fn parse_packet(packet: &str) -> Value {
    serde_json::from_str(packet).unwrap()
}

fn parse(input: &str) -> ItemPairs {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            pair.split("\n")
                .map(|packet| parse_packet(packet))
                .collect_tuple()
                .unwrap()
        })
        .collect::<ItemPairs>()
}

fn compare(pair: (Value, Value)) -> Ordering {
    match pair {
        (Value::Number(lhs), Value::Number(rhs)) => lhs.as_u64().cmp(&rhs.as_u64()),
        (Value::Array(lhs), Value::Array(rhs)) => {
            for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                let r = compare((lhs.clone(), rhs.clone()));
                if r != Ordering::Equal {
                    return r;
                }
            }
            return lhs.len().cmp(&rhs.len());
        }
        (Value::Number(lhs), Value::Array(rhs)) => {
            let new_lhs = Value::Array(vec![Value::Number(lhs)]);
            compare((new_lhs, Value::Array(rhs)))
        }
        (Value::Array(lhs), Value::Number(rhs)) => {
            let new_rhs = Value::Array(vec![Value::Number(rhs)]);
            compare((Value::Array(lhs), new_rhs))
        }
        _ => panic!("unknown pattern"),
    }
}

fn part_a(pairs: ItemPairs) -> usize {
    pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| {
            if compare(pair.clone()).is_le() {
                idx + 1
            } else {
                0
            }
        })
        .sum()
}

fn part_b(packets: &mut Vec<Value>) -> usize {
    let divider_1 = parse_packet("[[2]]");
    let divider_2 = parse_packet("[[6]]");

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort_by(|a, b| compare((a.clone(), b.clone())));
    packets
        .iter()
        .enumerate()
        .map(|(idx, packet)| {
            if packet == &divider_1 || packet == &divider_2 {
                idx + 1
            } else {
                1
            }
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn day13(input: &str) -> (String, String) {
    let mut all_packets = parse(&input)
        .iter()
        .flat_map(|val| vec![val.0.clone(), val.1.clone()])
        .collect::<Vec<_>>();
    (
        part_a(parse(&input)).to_string(),
        part_b(&mut all_packets).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_pair2() {
        let input = "[7,7,7,7]\n[7,7,7]";
        assert_eq!(part_a(parse(&input)), 0);
    }

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 13);
        assert_eq!(part_a(parse(&input.unwrap())), 13);
    }

    #[test]
    fn test_example_part_b() {
        let input = parse(&read_file("examples", 13).unwrap());
        let mut all_packets = input
            .iter()
            .flat_map(|val| vec![val.0.clone(), val.1.clone()])
            .collect::<Vec<_>>();

        assert_eq!(part_b(&mut all_packets), 140);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 13);
        assert_eq!(part_a(parse(&input.unwrap())), 5938);
    }

    #[test]
    fn test_input_part_b() {
        let input = parse(&read_file("inputs", 13).unwrap());
        let mut all_packets = input
            .iter()
            .flat_map(|val| vec![val.0.clone(), val.1.clone()])
            .collect::<Vec<_>>();

        assert_eq!(part_b(&mut all_packets), 29025);
    }
}
