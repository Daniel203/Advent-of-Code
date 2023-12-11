use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<Vec<isize>>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    return parsed_input;
}

fn resolve<F>(input: &InputType, f: F) -> isize
where
    F: Fn(isize, &Vec<isize>) -> isize,
{
    return input
        .iter()
        .map(|input_list| {
            let mut all_lists: Vec<Vec<isize>> = Vec::new();
            all_lists.push(input_list.to_owned());

            while !all_lists.last().unwrap().iter().all(|&x| x == 0) {
                let last_list = all_lists.last().unwrap();

                let deltas: Vec<isize> = last_list
                    .iter()
                    .skip(1)
                    .zip(last_list)
                    .map(|(x, prev)| x - prev)
                    .collect();

                all_lists.push(deltas);
            }

            all_lists
                .iter()
                .rev()
                .fold(0, |acc, list| f(acc, list))
        })
        .sum();
}

fn part_one(path: &str) -> isize {
    let data = parse_input(path);
    resolve(&data, |acc, list| list.last().unwrap() + acc)
}

fn part_two(path: &str) -> isize {
    let data = parse_input(path);
    resolve(&data, |acc, list| list.first().unwrap() - acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 2);
    }
}
