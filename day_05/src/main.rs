use std::fs;

const TEST_INPUT: &str = "src/inputs/test-input.txt";
const INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_1();
    let res2 = part_2();

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn read_data(path: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = fs::read_to_string(path).unwrap();

    let (stacks_raw, commands) = input.split_once("\n\n").expect("Invalid input");

    let (stacks_raw, idxs) = stacks_raw.rsplit_once('\n').expect("Invalid input");

    // parse stacks
    let mut stacks = vec![Vec::new(); idxs.split("   ").count()];
    for line in stacks_raw.lines().rev() {
        for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
            if chunk[1] != b' ' {
                stacks[i].push(char::from(chunk[1]));
            }
        }
    }

    // parse commands
    let commands: Vec<(usize, usize, usize)> = commands
        .lines()
        .map(|line| {
            let (n, rest) = line[5..].split_once(" from ").expect("Invalid input");
            let (from, to) = rest.split_once(" to ").expect("Invalid input");

            let n = n.parse().expect("Invalid input");
            let from = from.parse().expect("Invalid input");
            let to = to.parse().expect("Invalid input");

            (n, from, to)
        })
        .collect();

    return (stacks, commands);
}

fn part_1() -> String {
    let (mut stacks, commands) = read_data(INPUT);

    for (moves, from, to) in commands {
        for _ in 0..moves {
            let val = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(val);
        }
    }

    let mut output = String::from("");
    for stack in stacks {
        output.push(*(stack.last().unwrap()));
    }

    return output;
}

fn part_2() -> String {
    let (mut stacks, commands) = read_data(INPUT);

    for (moves, from, to) in commands {
        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..moves {
            tmp.push(stacks[from - 1].pop().unwrap());
        }

        for el in tmp.iter().rev() {
            stacks[to - 1].push(*el);
        }
    }

    let mut output = String::from("");

    for stack in stacks {
        output.push(*(stack.last().unwrap()));
    }

    return output;
}
