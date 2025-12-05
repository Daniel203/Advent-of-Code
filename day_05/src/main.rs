use std::{cmp, fs};

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
    let parsed_input = input
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    parsed_input
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut valid = std::collections::HashSet::new();

    data.iter().for_each(|l| {
        if l.contains("-") {
            let (l_str, r_str) = l.split_once("-").unwrap();
            let l = l_str.parse::<u64>().unwrap();
            let r = r_str.parse::<u64>().unwrap();

            ranges.push((l, r));
        } else {
            let n = l.parse::<u64>().unwrap();

            ranges.iter().for_each(|(l, r)| {
                if &n >= l && &n <= r {
                    valid.insert(n);
                }
            });
        }
    });

    valid.len()
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let data = parse_input(path);

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    data.iter().for_each(|l| {
        if l.contains("-") {
            let (l_str, r_str) = l.split_once("-").unwrap();
            let l = l_str.parse::<u64>().unwrap();
            let r = r_str.parse::<u64>().unwrap();

            let mut new_range = (l, r);
            let mut to_delete = Vec::new();

            for (i, (l_range, r_range)) in ranges.iter().enumerate() {
                if new_range.0 <= *r_range && new_range.1 >= *l_range {
                    new_range.0 = new_range.0.min(*l_range);
                    new_range.1 = new_range.1.max(*r_range);
                    to_delete.push(i);
                }
            }

            for (count_removed, i_to_delete) in to_delete.into_iter().enumerate() {
                ranges.remove(i_to_delete - count_removed);
            }

            ranges.push(new_range);
        }
    });

    let mut res = 0;
    for (l, r) in ranges {
        res += r+1 - l
    }

    res as usize
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
        assert_eq!(part_two(INPUT_TEST_DATA), 14);
    }
}
