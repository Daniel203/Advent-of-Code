use std::{collections::HashMap, fs};
use num::integer::lcm;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = (String, HashMap<String, Vec<String>>);

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let split = input
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let instructions = &split[0];

    let mut netwok = HashMap::new();
    let mut start_key = "".to_string();

    split[1].split("\n").for_each(|x| {
        if x.is_empty() {
            return;
        }
        let line_split = x
            .split(" = ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let key = &line_split[0].clone();

        let values = line_split[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        netwok.insert(key.clone(), values);
    });

    return (instructions.clone(), netwok);
}

fn part_one(path: &str) -> usize {
    let (instructions, network) = parse_input(path);

    let mut key = "AAA".to_string();
    let mut i = 0;
    let mut count = 0;

    while key != "ZZZ" {
        let direction = &instructions.chars().nth(i).unwrap();
        let direction_index = if *direction == 'L' { 0 } else { 1 };

        if i + 1 < instructions.len() {
            i += 1;
        } else {
            i = 0;
        }

        key = network.get(&key).unwrap()[direction_index].clone();
        count += 1;
    }

    return count;
}

fn part_two(path: &str) -> usize {
    let (instructions, network) = parse_input(path);

    let keys = network
        .keys()
        .filter(|x| x.chars().last() == Some('A'))
        .collect::<Vec<&String>>();

    let mut counts = vec![];

    for x in keys {
        let mut key = x.clone();
        let mut i = 0;
        let mut count = 0;

        while !key.ends_with("Z"){
            let direction = &instructions.chars().nth(i).unwrap();
            let direction_index = if *direction == 'L' { 0 } else { 1 };

            if i + 1 < instructions.len() {
                i += 1;
            } else {
                i = 0;
            }

            key = network.get(&key).unwrap()[direction_index].clone();
            count += 1;
        }

        counts.push(count);
    }

    let mut result = counts[0];
    counts.iter().for_each(|x| {
        result = lcm(result, *x);
    });

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";
    static INPUT_TEST_DATA_2: &str = "src/inputs/test_input_2.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 6);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA_2), 6);
    }
}
