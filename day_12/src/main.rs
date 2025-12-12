use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = (Vec<usize>, Vec<(usize, Vec<usize>)>);

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let (presents_str, regions_str) = input.rsplit_once("\n\n").unwrap();

    let presents = presents_str
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.matches("#").count()).sum())
        .collect();

    let regions = regions_str
        .lines()
        .map(|x| {
            let (dimensions_str, indices_str) = x.split_once(":").unwrap();

            let (l, r) = dimensions_str.split_once("x").unwrap();
            let dimension = l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap();

            let indeces = indices_str
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            (dimension, indeces)
        })
        .collect();

    (presents, regions)
}

fn part_one(path: &str) -> usize {
    let (presents, regions) = parse_input(path);
    let mut res = 0;

    for (dimension, required_presents) in regions {
        let mut presents_sum = 0;
        for (present_idx, present_count) in required_presents.iter().enumerate() {
            presents_sum += presents[present_idx] * present_count;
        }

        if presents_sum <= dimension {
            res += 1;
        }
    }

    res
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 0);
    }
}
