use std::collections::{BinaryHeap, HashMap};
use std::fs;

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
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    return parsed_input;
}

// dijikstra
fn solve(grid: &InputType, min_steps: isize, max_steps: isize)-> isize {
    let start = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);

    let mut dists = HashMap::new(); // (y, x, dir) -> cost
    let mut heap = BinaryHeap::from_iter([(0, (start.0, start.1, (0, 0)))]); // (cost, (y, x, dir)

    while let Some((cost, (y, x, dir))) = heap.pop() {
        // found the end
        if (y, x) == end {
            return -cost;
        }

        // if we already have a better path to this node, skip it
        if dists.get(&(y, x, dir)).is_some_and(|&c| -cost > c) {
            continue;
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // can't go back
            if dir == (dy, dx) || dir == (-dy, -dx) {
                continue;
            }

            let mut next_cost = -cost;

            // move forward max n steps
            for dist in 1..=max_steps {
                let ny = (y as isize + dy * dist) as usize;
                let nx = (x as isize + dx * dist) as usize;

                // check out of bounds
                if ny >= grid.len() || nx >= grid[0].len() {
                    continue;
                }

                // update the cost with the new value
                next_cost += grid[ny][nx].to_digit(10).unwrap() as isize;

                if dist < min_steps {
                    continue;
                }

                // check if the cost is less than the current one
                let key = (ny, nx, (dy, dx));
                if next_cost < *dists.get(&key).unwrap_or(&isize::MAX) {
                    // update the cost and push it to the heap
                    dists.insert(key, next_cost);
                    heap.push((-next_cost, key));
                }
            }
        }
    }

    return 0;
}

fn part_one(path: &str) -> isize {
    let data = parse_input(path);

    let min_blocks = 1;
    let max_blocks = 3;

    return solve(&data, min_blocks, max_blocks);
}

fn part_two(path: &str) -> isize {
    let data = parse_input(path);

    let min_blocks = 4;
    let max_blocks = 10;

    return solve(&data, min_blocks, max_blocks);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 102);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 94);
    }
}
