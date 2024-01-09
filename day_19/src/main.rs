use itertools::Itertools;
use std::{collections::HashMap, fs};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType<'a> = (
    HashMap<String, Vec<(usize, bool, usize, String)>>,
    Vec<[usize; 4]>,
);

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let (workflows, ratings) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, instructions) = line[..line.len() - 1].split_once('{').unwrap();

            let instructions = instructions
                .split(",")
                .map(|instruction| {
                    if !instruction.contains(":") {
                        return (0, false, usize::MAX - 1, instruction.to_string());
                    }

                    let category = match instruction.as_bytes()[0] {
                        b'x' => 0,
                        b'm' => 1,
                        b'a' => 2,
                        b's' => 3,
                        _ => panic!("Invalid rating!"),
                    };
                    let is_greater = instruction.as_bytes()[1] == b'>';
                    let (value, next) = instruction[2..instruction.len()].split_once(':').unwrap();

                    return (
                        category,
                        is_greater,
                        value.parse().unwrap(),
                        next.to_string(),
                    );
                })
                .collect();

            return (name.to_string(), instructions);
        })
        .collect();

    let ratings = ratings
        .lines()
        .map(|line| {
            let (_, x, _, m, _, a, _, s, _) =
                line.split(&['=', ',', '}'][..]).collect_tuple().unwrap();
            return [x, m, a, s].map(|n| n.parse().unwrap());
        })
        .collect();

    return (workflows, ratings);
}

fn part_one(path: &str) -> usize {
    let (workflows, ratings) = parse_input(path);

    return ratings
        .iter()
        .filter(|rating| {
            let mut workflow = "in";

            loop {
                for (category, is_greater, value, next) in &workflows[workflow] {
                    let condition = if *is_greater {
                        rating[*category] > *value
                    } else {
                        rating[*category] < *value
                    };

                    if condition {
                        match next.as_str() {
                            "A" => return true,
                            "R" => return false,
                            next => workflow = next,
                        }
                        break;
                    }
                }
            }
        })
        .flatten()
        .sum();
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 19114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 167409079868000);
    }
}
