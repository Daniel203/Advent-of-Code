use std::fs;

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);

    println!("1: {}", res1);
    part_two(INPUT);
}

fn part_one(path: &str) -> i32 {
    let input = fs::read_to_string(path).unwrap();

    let mut cycle = 1;
    let mut strenght = 0;
    let mut register = 1;

    for line in input.lines() {
        cycle += 1;

        if line.contains("addx") {
            let (_, value_str) = line.split_once(" ").unwrap();
            let value: i32 = value_str.parse().unwrap();

            if cycle % 40 == 20 {
                strenght += cycle * register;
            }

            cycle += 1;
            register += value;
        }

        if cycle % 40 == 20 {
            strenght += cycle * register;
        }
    }

    return strenght;
}

fn part_two(path: &str) {
    let input = fs::read_to_string(path).unwrap();

    let mut sprite = 1;
    let mut cycle = 1;

    for line in input.lines() {
        if sprite <= cycle % 40 && cycle % 40 <= sprite + 2 {
            print!("█");
        } else {
            print!(" ");
        }

        cycle += 1;

        if line.contains("addx") {
            let value: i32 = line.split_once(" ").unwrap().1.parse().unwrap();

            if cycle % 40 == 1 {
                print!("\n");
            }

            if sprite <= cycle % 40 && cycle % 40 <= sprite + 2 {
                print!("█");
            } else {
                print!(" ");
            }

            cycle += 1;
            sprite += value;
        }

        if cycle % 40 == 1 {
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 13140);
    }
}
