use std::collections::HashSet;
use std::fs;

static TEST_INPUT_1: &str = "src/inputs/test_input_1.txt";
static TEST_INPUT_2: &str = "src/inputs/test_input_2.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn get_direction_from_string(command: &str) -> (isize, isize) {
    match command {
        "U" => (0, -1),
        "D" => (0, 1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => (0, 0),
    }
}

fn get_commands(data: &str) -> Vec<((isize, isize), usize)> {
    return data
        .lines()
        .into_iter()
        .map(|line| {
            let (direction_str, amount_str) = line.split_once(" ").unwrap();
            return (
                get_direction_from_string(direction_str),
                usize::from_str_radix(amount_str, 10).unwrap(),
            );
        })
        .collect();
}

fn process(n_knots: usize, input: &str) -> usize {
    let commands = get_commands(input);
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); n_knots];

    for command in commands {
        let (x_move, y_move) = command.0;

        for _ in 0..command.1 {
            rope[0] = (rope[0].0 + x_move, rope[0].1 + y_move);

            for i in 0..n_knots - 1 {
                let head = rope[i];
                let mut tail = rope[i + 1];

                let (delta_x, delta_y) = (head.0 - tail.0, head.1 - tail.1);

                if i32::pow(delta_x as i32, 2) + i32::pow(delta_y as i32, 2) > 2 {
                    tail.0 += delta_x.signum();
                    tail.1 += delta_y.signum();
                    rope[i + 1] = (tail.0, tail.1);
                }
            }

            seen.insert(rope[n_knots - 1]);
        }
    }

    return seen.len();
}

fn part_one(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();

    return process(2, &input);
}

fn part_two(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();

    return process(10, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT_1), 13);
    }

    #[test]
    fn test_part_two_input_1() {
        assert_eq!(part_two(TEST_INPUT_1), 1);
    }

    #[test]
    fn test_part_two_input_2() {
        assert_eq!(part_two(TEST_INPUT_2), 36);
    }
}
