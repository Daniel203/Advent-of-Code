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

fn check_invalid_ids_one(id: u64) -> bool {
    let id_str = id.to_string();
    let bytes = id_str.as_bytes();

    // Return false if odd number
    if bytes.len() % 2 == 1 {
        return false;
    }

    let half = bytes.len() / 2;
    for i in 0..half {
        if bytes[i] != bytes[i + half] {
            return false;
        }
    }

    true
}

fn check_invalid_ids_two(id: u64) -> bool {
    let id_str = id.to_string();
    let n = id_str.len();

    for k in 1..n {
        if !n.is_multiple_of(k) {
            continue;
        }

        let pattern = &id_str[..k];
        let repeat_count = n / k;

        let built: String = pattern.repeat(repeat_count);

        if built == id_str {
            return true;
        }

    }

    false
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    data
        .iter()
        .filter(|l| !l .is_empty())
        .flat_map(|l| {
            l.split(',')
                .map(|pair| {
                    let (a, b) = pair.split_once('-').unwrap();
                    let a = a.parse::<u64>().unwrap();
                    let b = b.parse::<u64>().unwrap();

                    let mut sum = 0;
                    for id in a..b+1 {
                        if check_invalid_ids_one(id) {
                            sum += id;
                        }
                    }

                    sum
                })
        })
        .sum::<u64>() as usize
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    data
        .iter()
        .filter(|l| !l .is_empty())
        .flat_map(|l| {
            l.split(',')
                .map(|pair| {
                    println!("pair: {pair}");
                    let (a, b) = pair.split_once('-').unwrap();
                    let a = a.parse::<u64>().unwrap();
                    let b = b.parse::<u64>().unwrap();

                    let mut sum = 0;
                    for id in a..b+1 {
                        if check_invalid_ids_two(id) {
                            sum += id;
                        }
                    }

                    sum
                })
        })
        .sum::<u64>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 4174379265);
    }
}
