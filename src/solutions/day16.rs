use std::fs;

use core::slice::Iter;
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Packet {
    version: u16,
    type_id: ExpressionType,
    data: Data,
}

#[derive(Debug, Clone, PartialEq)]
enum ExpressionType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Greater,
    Less,
    Equal,
}

#[derive(Debug, Clone)]
enum Data {
    Literal(u64),
    Operator(Vec<Packet>),
}

fn to_bits(hex_data: &Vec<u8>) -> Vec<bool> {
    let mut bits = vec![];
    for hex in hex_data {
        let s = format!("{:b}", hex);
        let mut chars = s.chars().map(|c| c == '1').collect::<Vec<_>>();
        while chars.len() < 8 {
            chars.insert(0, false);
        }
        bits.append(&mut chars);
    }
    bits
}

fn to_u16(bits: &Vec<&bool>) -> u16 {
    bits.iter()
        .fold(0, |result, &bit| (result << 1) ^ (*bit as u16))
}

fn parse_packet(bit_stream: &mut Iter<bool>) -> Packet {
    let version = to_u16(&bit_stream.take(3).collect::<Vec<_>>());
    let type_id = match to_u16(&bit_stream.take(3).collect::<Vec<_>>()) {
        0 => ExpressionType::Sum,
        1 => ExpressionType::Product,
        2 => ExpressionType::Min,
        3 => ExpressionType::Max,
        4 => ExpressionType::Literal,
        5 => ExpressionType::Greater,
        6 => ExpressionType::Less,
        7 => ExpressionType::Equal,
        _ => unreachable!("Invalid expression type"),
    };

    //println!("Version: {}", version);
    //println!("Type ID: {}", type_id);

    let data = if type_id == ExpressionType::Literal {
        let mut num = 0u64;

        loop {
            let bits = bit_stream.by_ref().take(5).collect::<Vec<_>>();
            let bit = to_u16(&bits);

            num <<= 4;
            num |= (bit as u64) & 0b1111;

            let leftmost = (bit >> 4) & 1;
            if leftmost == 0 {
                break;
            }
        }
        Data::Literal(num)
    } else {
        let length_type = to_u16(&bit_stream.by_ref().take(1).collect::<Vec<_>>());
        //println!("Length Type ID: {:?}", length_type);
        let mut packets: Vec<Packet> = vec![];

        if length_type == 0 {
            let length = to_u16(&bit_stream.by_ref().take(15).collect::<Vec<_>>());
            if length > 0 {
                //println!("Length: {:b} {}", length, length);

                let the_data = bit_stream
                    .by_ref()
                    .take(length.into())
                    .map(|v| *v)
                    .collect::<Vec<_>>();

                let mut it = the_data.iter();

                while it.size_hint().0 > 0 {
                    packets.push(parse_packet(&mut it));
                }
            }
        } else {
            let length = to_u16(&bit_stream.by_ref().take(11).collect::<Vec<_>>());
            if length > 0 {
                //println!("Length: {:b} {}", length, length);

                for _ in 0..length {
                    packets.push(parse_packet(bit_stream));
                }
            }
        }
        Data::Operator(packets)
    };
    Packet {
        version,
        type_id,
        data,
    }
}

fn compute_sum(packet: Packet) -> u64 {
    match packet.data {
        Data::Literal(_) => packet.version as u64,
        Data::Operator(subpackets) => {
            packet.version as u64
                + subpackets
                    .iter()
                    .fold(0, |accum, elem| accum + compute_sum(elem.clone()))
        }
    }
}

fn eval_packet(packet: Packet) -> u64 {
    match packet.data {
        Data::Literal(val) => {
            if packet.type_id == ExpressionType::Literal {
                val
            } else {
                panic!("Invalid literal type")
            }
        }
        Data::Operator(subpackets) => {
            let mut evaluated_packets = subpackets.into_iter().map(|p| eval_packet(p));

            match packet.type_id {
                ExpressionType::Sum => evaluated_packets.sum(),
                ExpressionType::Product => evaluated_packets.product(),
                ExpressionType::Min => evaluated_packets.min().unwrap(),
                ExpressionType::Max => evaluated_packets.max().unwrap(),
                ExpressionType::Greater => {
                    (evaluated_packets.nth(0) > evaluated_packets.nth(0)) as u64
                }
                ExpressionType::Less => {
                    (evaluated_packets.nth(0) < evaluated_packets.nth(0)) as u64
                }
                ExpressionType::Equal => {
                    (evaluated_packets.nth(0) == evaluated_packets.nth(0)) as u64
                }
                _ => unreachable!("Invalid expression type for operator"),
            }
        }
    }
}

fn parse_data(input: &str) -> Vec<bool> {
    let char_data: Vec<char> = input.chars().into_iter().collect();
    let decoded_data: Vec<u8> = char_data
        .chunks(2)
        .filter_map(|v| u8::from_str_radix(&v.iter().collect::<String>(), 16).ok())
        .collect();
    to_bits(&decoded_data)
}

pub fn day16() {
    let filename = "src/inputs/day16.txt";

    let file = fs::read_to_string(filename).expect("Unable to read file");
    let hex_data = file.lines().into_iter().nth(0).unwrap();
    let binary_data = parse_data(hex_data);
    let packet = parse_packet(&mut binary_data.iter());

    println!("{}", compute_sum(packet.clone()));
    println!("{}", eval_packet(packet));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_sum() {
        let test_cases = [
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("EE00D40C823060", 14),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for (input, expected) in test_cases {
            let data = parse_data(input);
            let packet = parse_packet(&mut data.iter());
            assert_eq!(compute_sum(packet), expected);
        }
    }

    #[test]
    fn eval_packets() {
        let test_cases = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (input, expected) in test_cases {
            let data = parse_data(input);
            let packet = parse_packet(&mut data.iter());
            assert_eq!(eval_packet(packet), expected);
        }
    }
}
