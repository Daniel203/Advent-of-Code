use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<String>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input.split("\n").map(|x| x.to_string()).collect();
    parsed_input
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let mut num: i32 = 50;
    let mut res = 0;

    for line in data {
        if line.is_empty() {
            continue;
        }

        let (direction, amount_str) = line.split_at(1);
        let amount = amount_str
            .parse::<i32>()
            .expect("Failed to convert from string to u32");

        if direction == "L" {
            num = (num - amount).rem_euclid(100);
        } else if direction == "R" {
            num = (num + amount).rem_euclid(100);
        }

        if num == 0 {
            res += 1;
        }
    }

    res
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let mut num: i32 = 50;
    let mut res = 0;

    for line in data {
        if line.is_empty() {
            continue;
        }

        let (direction, amount_str) = line.split_at(1);
        let amount = amount_str.parse::<i32>().unwrap();
        let step = if direction == "L" { -1 } else { 1 };

        for _ in 0..amount {
            num += step;

            if num == 0 || num == 100{
                res += 1
            }

            num = num.rem_euclid(100);
        }

    }

    res 
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 6);
    }
}
