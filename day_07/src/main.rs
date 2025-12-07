use std::{
    collections::{HashSet, VecDeque},
    fs,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<Vec<char>>;

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
        .map(|x| x.chars().collect())
        .collect();
    parsed_input
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let start_y = 0;
    let start_x = data[start_y].iter().position(|&c| c == 'S').unwrap();
    let start = (start_y, start_x);

    let mut q = VecDeque::new();
    q.push_back(start);

    let mut visited = HashSet::new();

    let mut count_splits = 0;

    while !q.is_empty() {
        let (y, x) = q.pop_front().unwrap();

        if !visited.insert((y, x)) {
            continue;
        }

        if data[y][x] == '^' {
            count_splits += 1;
            if y == data.len() - 1 {
                continue;
            }

            if x >= 1 {
                let (ny_left, nx_left) = (y, x - 1);
                q.push_back((ny_left, nx_left));
            }

            if x < data[0].len() - 1 {
                let (ny_right, nx_right) = (y, x + 1);
                q.push_back((ny_right, nx_right));
            }
        } else {
            let (ny, nx) = (y + 1, x);
            if ny < data.len() {
                q.push_back((ny, nx));
            }
        }
    }

    count_splits
}

fn count_paths(
    y: usize,
    x: usize,
    data: &Vec<Vec<char>>,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // If we reached the bottom (or past it), we found 1 valid end.
    if y >= data.len() - 1 {
        return 1;
    }

    // Have we solved this exact spot before?
    if let Some(count) = memo[y][x] {
        return count;
    }

    // Calculate based on logic
    let mut total_paths = 0;
    let current_char = data[y][x];

    if current_char == '^' {
        if x > 0 {
            total_paths += count_paths(y, x - 1, data, memo);
        }

        if x < data[0].len() - 1 {
            total_paths += count_paths(y, x + 1, data, memo);
        }
    } else {
        total_paths += count_paths(y + 1, x, data, memo);
    }

    memo[y][x] = Some(total_paths);

    total_paths
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let height = data.len();
    let width = data[0].len();

    let start_y = 0;
    let start_x = data[start_y].iter().position(|&c| c == 'S').unwrap();

    // The Cache: Stores the result for (y, x) so we don't recalculate it
    let mut memo = vec![vec![None; width]; height];

    count_paths(start_y, start_x, &data, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 40);
    }
}
