use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = HashMap<String, Vec<String>>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let mut output = InputType::new();

    input.lines().for_each(|line| {
        let (from, to_list) = line.split_once(":").unwrap();

        let iter = to_list.split_whitespace();
        for to in iter {
            output
                .entry(from.to_string())
                .or_default()
                .push(to.to_string());
        }
    });

    output
}

fn count_paths(board: &InputType, start: &str, end: &str, required: &[&str]) -> usize {
    fn dfs(
        board: &HashMap<String, Vec<String>>,
        current: &str,
        end: &str,
        required: &HashSet<String>,
        visited_required: &mut HashSet<String>,
        memo: &mut HashMap<(String, Vec<String>), usize>,
    ) -> usize {
        // Update visited required nodes
        if required.contains(current) {
            visited_required.insert(current.to_string());
        }

        // Build memo key
        let mut key_vec: Vec<String> = visited_required.iter().cloned().collect();
        key_vec.sort();
        let key = (current.to_string(), key_vec);

        // Check memo
        if let Some(&v) = memo.get(&key) {
            // remove current from visited_required before returning
            if required.contains(current) {
                visited_required.remove(current);
            }
            return v;
        }

        // Base case: reached end
        let total = if current == end {
            if visited_required.len() == required.len() {
                1
            } else {
                0
            }
        } else {
            // Recurse over neighbors
            let mut sum = 0;
            if let Some(neighbors) = board.get(current) {
                for n in neighbors {
                    sum += dfs(board, n, end, required, visited_required, memo);
                }
            }
            sum
        };

        memo.insert(key, total);

        // Backtrack
        if required.contains(current) {
            visited_required.remove(current);
        }

        total
    }

    let required_set: HashSet<String> = required.iter().map(|s| s.to_string()).collect();
    dfs(
        board,
        start,
        end,
        &required_set,
        &mut HashSet::new(),
        &mut HashMap::new(),
    )
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    count_paths(&data, "you", "out", &[])
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    count_paths(&data, "svr", "out", &["dac", "fft"])
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 2);
    }
}
