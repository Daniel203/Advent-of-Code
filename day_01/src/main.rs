use std::fs;

static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
    parsed_input
}

fn part_one(path: &str) -> usize {
    let data: Vec<String> = parse_input(path);
    let output: usize = data
        .iter()
        .map(|line| {
            let num_1: char = line
                .chars()
                .into_iter()
                .skip_while(|ch| !ch.is_digit(10))
                .next()
                .unwrap_or_default();

            let num_2: char = line
                .chars()
                .into_iter()
                .rev()
                .skip_while(|ch| !ch.is_digit(10))
                .next()
                .unwrap_or_default();

            return format!("{num_1}{num_2}")
                .parse::<usize>()
                .unwrap_or_default();
        })
        .sum();
    return output;
}

fn part_two(path: &str) -> usize {
    let data: Vec<String> = parse_input(path);

    let output: usize = data
        .iter()
        .map(|line| {
            let mut num_1: u32 = 0;

            (0..line.len()).for_each(|i| {
                let reduced_string = line.get(i..line.len()).unwrap().to_string();
                if num_1 == 0 {
                    if reduced_string.chars().nth(0).unwrap().is_digit(10) {
                        num_1 = reduced_string.chars().nth(0).unwrap().to_digit(10).unwrap();
                    } else if reduced_string.starts_with("one") {
                        num_1 = 1;
                    } else if reduced_string.starts_with("two") {
                        num_1 = 2;
                    } else if reduced_string.starts_with("three") {
                        num_1 = 3;
                    } else if reduced_string.starts_with("four") {
                        num_1 = 4;
                    } else if reduced_string.starts_with("five") {
                        num_1 = 5;
                    } else if reduced_string.starts_with("six") {
                        num_1 = 6;
                    } else if reduced_string.starts_with("seven") {
                        num_1 = 7;
                    } else if reduced_string.starts_with("eight") {
                        num_1 = 8;
                    } else if reduced_string.starts_with("nine") {
                        num_1 = 9;
                    }
                }
            });

            let mut num_2: u32 = 0;

            (0..line.len()).rev().for_each(|i| {
                let reduced_string = line.get(i..line.len()).unwrap().to_string();

                if num_2 == 0 {
                    if reduced_string.chars().nth(0).unwrap().is_digit(10) {
                        num_2 = reduced_string.chars().nth(0).unwrap().to_digit(10).unwrap();
                    } else if reduced_string.starts_with("one") {
                        num_2 = 1;
                    } else if reduced_string.starts_with("two") {
                        num_2 = 2;
                    } else if reduced_string.starts_with("three") {
                        num_2 = 3;
                    } else if reduced_string.starts_with("four") {
                        num_2 = 4;
                    } else if reduced_string.starts_with("five") {
                        num_2 = 5;
                    } else if reduced_string.starts_with("six") {
                        num_2 = 6;
                    } else if reduced_string.starts_with("seven") {
                        num_2 = 7;
                    } else if reduced_string.starts_with("eight") {
                        num_2 = 8;
                    } else if reduced_string.starts_with("nine") {
                        num_2 = 9;
                    }
                }
            });

            return format!("{num_1}{num_2}")
                .parse::<usize>()
                .unwrap_or_default();
        })
        .sum();

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "src/inputs/test_input.txt";
    static TEST_INPUT_2: &str = "src/inputs/test_input_2.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT_2), 281);
    }
}
