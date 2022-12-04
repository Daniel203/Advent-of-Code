use std::collections::HashSet;
use std::fs;

const TEST_INPUT: &str = "src/test_input.txt";
const INPUT: &str = "src/input.txt";

fn main() {
    let res1 = part1();
    let res2 = part2();

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn part1() -> i32 {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut chars: Vec<char> = Vec::new();

    for line in contents.lines() {
        let len = line.chars().count();
        let first_half: HashSet<char> = line.get(0..(len / 2)).unwrap().chars().collect();
        let second_half: HashSet<char> = line.get((len / 2)..len).unwrap().chars().collect();

        let common_chars: Vec<&char> = first_half.intersection(&second_half).collect();

        for x in common_chars {
            chars.push(*x);
        }
    }

    let mut sum: i32 = 0;

    for x in chars {
        let x: u8 = x as u8;

        match x {
            b'a'..=b'z' => sum += (x - b'a' + 1) as i32,
            b'A'..=b'Z' => sum += (x - b'A' + 27) as i32,
            _ => panic!(),
        }
    }

    return sum;
}

fn part2() -> i32 {
    let contents = fs::read_to_string(INPUT).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut i = 0;
    let mut chars: Vec<char> = Vec::new();

    while i < lines.len() - 2 {
        let line1: HashSet<char> = lines[i].chars().collect();
        let line2: HashSet<char> = lines[i + 1].chars().collect();
        let line3: HashSet<char> = lines[i + 2].chars().collect();

        let common_chars_1_vec: Vec<&char> = line1.intersection(&line2).collect();

        let common_chars_1_hashset: HashSet<char> =
            HashSet::from_iter(common_chars_1_vec.iter().cloned().cloned());

        let common_chars: Vec<&char> = common_chars_1_hashset.intersection(&line3).collect();

        for x in common_chars {
            chars.push(*x);
        }

        i += 3
    }

    let mut sum: i32 = 0;

    for x in chars {
        let x: u8 = x as u8;

        match x {
            b'a'..=b'z' => sum += (x - b'a' + 1) as i32,
            b'A'..=b'Z' => sum += (x - b'A' + 27) as i32,
            _ => panic!(),
        }
    }

    return sum;
}
