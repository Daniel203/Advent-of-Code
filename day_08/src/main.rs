use num::integer::lcm;
use std::{collections::HashMap, fs};

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

fn count_n_steps(key: String, data: &InputType) -> usize {
    let (instructions, network) = data;
    let mut key = key;
    let mut i = 0;
    let mut count = 0;

    while !key.ends_with("Z") {
        let direction = &instructions.chars().nth(i).unwrap();
        let direction_index = if *direction == 'L' { 0 } else { 1 };
        i = if i + 1 < instructions.len() { i + 1 } else { 0 };

        key = network.get(&key).unwrap()[direction_index].clone();
        count += 1;
    }

    return count;
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    return count_n_steps("AAA".to_string(), &data);
}

fn part_two(path: &str) -> usize {
    let (instructions, network) = parse_input(path);

    let counts = network
        .keys()
        .filter(|x| x.chars().last() == Some('A'))
        .map(|key| count_n_steps(key.clone(), &(instructions.clone(), network.clone())))
        .collect::<Vec<usize>>();

    let mut result = counts[0];
    counts.iter().for_each(|x| {
        result = lcm(result, *x);
    });

    return counts.iter().fold(counts[0], |acc, x| lcm(acc, *x));
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
