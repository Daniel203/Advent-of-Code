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

    data
        .iter()
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let mut first_digit = 0;
            let mut pos_first_digit = -1;
            let mut second_digit = 0;

            for i in 0..l.len() - 1 {
                let digit: u32 = l.chars().nth(i).unwrap().to_digit(10).unwrap();
                if digit > first_digit {
                    pos_first_digit = i as isize;
                    first_digit = digit;
                }
            }

            for j in (pos_first_digit as usize)+1..l.len() {
                let digit: u32 = l.chars().nth(j).unwrap().to_digit(10).unwrap();
                if digit > second_digit {
                    second_digit = digit;
                }
            }

            (first_digit * 10 + second_digit) as usize
        })
        .sum()
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let data = parse_input(path);

    data
        .iter()
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let mut output = 0;
            let mut last_index = 0;

            for i in (0..=12-1).rev() {
                let mut max_digit = 0;
                let mut max_index = 0;

                for j in last_index..l.len()-i {
                    let digit: u64 = l.chars().nth(j).unwrap().to_digit(10).unwrap() as u64;
                    if digit > max_digit {
                        max_digit = digit;
                        max_index = j;
                    }
                }

                last_index = max_index + 1;
                output += max_digit * 10u64.pow(i as u32);
            }

            println!("output: {output}");
            output as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 357);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 3121910778619);
    }
}
