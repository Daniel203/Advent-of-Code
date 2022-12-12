use std::fs;

const TEST_INPUT: &str = "src/inputs/test_input.txt";
const INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part1();
    let res2 = part2();

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn part1() -> u32 {
    let mut sum = 0;

    let mut stack = vec![("/", 0)];

    for line in fs::read_to_string(INPUT).unwrap().lines() {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = line.rsplit_once(" ").unwrap().1;

            if dir == ".." {
                let (_, amount) = stack.pop().unwrap();
                if amount <= 100_000 {
                    sum += amount;
                }
                stack.last_mut().unwrap().1 += amount;
            } else {
                stack.push((dir, 0));
            }
        } else {
            let (amount, _) = line.split_once(" ").unwrap();

            if amount != "dir" {
                stack.last_mut().unwrap().1 += amount.parse::<u32>().unwrap();
            }
        }
    }

    return sum;
}

fn part2() -> u32 {
    let input = fs::read_to_string(INPUT).unwrap();

    let mut stack = vec![("/", 0)];
    let mut dirs = vec![];

    for line in input.lines() {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = line.rsplit_once(" ").unwrap().1;

            if dir == ".." {
                let (name, amount) = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += amount;
                dirs.push((name, amount));
            } else {
                stack.push((dir, 0));
            }
        } else {
            let (amount, _) = line.split_once(" ").unwrap();

            if amount != "dir" {
                stack.last_mut().unwrap().1 += amount.parse::<u32>().unwrap();
            }
        }
    }

    while stack.len() > 0 {
        let (name, amount) = stack.pop().unwrap();
        dirs.push((name, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let free_space = 70_000_000 - dirs.last().unwrap().1;
    let space_needed = 30_000_000 - free_space;

    let out = dirs
        .into_iter()
        .filter(|(_, amount)| amount >= &space_needed)
        .map(|(_, amount)| amount)
        .min()
        .unwrap();

    return out;
}
