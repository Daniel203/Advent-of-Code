use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(input: &str) -> Result<Packet, Self::Err> {
        let chars: Vec<char> = input.chars().collect();

        let (packet, _) = parse_packet_list(&chars);

        return Ok(packet);
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_num_packet(list: &[char]) -> (Packet, &[char]) {
    // find the index where the first number ends
    let mut i = 0;
    while i < list.len() && list[i].is_ascii_digit() {
        i += 1;
    }

    // parse the number
    let mut num = 0;
    let mut offset = 1;
    for c in list[..i].iter().rev() {
        num += (*c as u8 - b'0') * offset;
        offset *= 10;
    }

    // return the number and the rest of the packet
    return (Packet::Num(num), &list[i..]);
}

fn parse_packet_list(chars: &[char]) -> (Packet, &[char]) {
    // remove first char
    let mut list = &chars[1..];

    let mut packets = Vec::new();

    loop {
        match list[0] {
            ']' => {
                break;
            }
            '[' => {
                let (packet, rest) = parse_packet_list(list);
                packets.push(packet);
                list = rest;
            }
            ',' => {
                list = &list[1..];
            }
            _ => {
                let (num, rest) = parse_num_packet(list);
                packets.push(num);
                list = rest;
            }
        }
    }

    return (Packet::List(packets), &list[1..]);
}

fn get_pairs(input: &str) -> Vec<[Packet; 2]> {
    return input
        .split("\n\n")
        .into_iter()
        .map(|pair| {
            let pair_splitted = pair.split_once("\n").unwrap();

            // parse the two packets
            let left = pair_splitted.0.parse::<Packet>().unwrap();
            let right = pair_splitted.1.parse::<Packet>().unwrap();

            return [left, right];
        })
        .collect();
}

fn parse_input(path: &str) -> Vec<[Packet; 2]> {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    return get_pairs(&input);
}

fn part_one(path: &str) -> usize {
    let input = parse_input(path);

    let out = input
        .iter()
        .positions(|[left, right]| left < right)
        .map(|idx| idx + 1)
        .sum();

    return out;
}

fn part_two(path: &str) -> usize {
    let input = parse_input(path);
    let mut packets: Vec<_> = input.iter().flatten().collect();

    let divider_1 = "[[2]]".parse::<Packet>().unwrap();
    let divider_2 = "[[6]]".parse::<Packet>().unwrap();

    packets.push(&divider_1);
    packets.push(&divider_2);

    packets.sort_unstable();

    let out = packets
        .into_iter()
        .positions(|packet| packet == &divider_1 || packet == &divider_2)
        .map(|idx| idx + 1)
        .product();

    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 140);
    }
}
