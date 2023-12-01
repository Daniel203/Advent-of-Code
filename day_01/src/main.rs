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
    return 0
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
