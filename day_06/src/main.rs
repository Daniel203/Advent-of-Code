use std::{fs, process::Output};

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

    let mut numbers: Vec<usize> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    data.iter().rev().enumerate().for_each(|(i, line)| {
        let iter = line.split_whitespace();
        if i == 0 {
            // Operations
            operations = iter.map(|s| s.chars().next().unwrap()).collect();
        } else {
            // Numbers
            iter.enumerate().for_each(|(i, s)| {
                let num = s.parse::<usize>().unwrap();
                if operations[i] == '+' {
                    if let Some(value) = numbers.get_mut(i) {
                        *value += num;
                    } else {
                        numbers.push(num);
                    }
                } else if operations[i] == '*' {
                    if let Some(value) = numbers.get_mut(i) {
                        *value *= num;
                    } else {
                        numbers.push(num);
                    }
                }
            })
        }
    });

    numbers.iter().sum()
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let last = data.last().unwrap();

    // Collect operations with their column index
    let operations: Vec<(usize, char)> = last
        .char_indices()
        .filter(|&(_, c)| c == '+' || c == '*')
        .collect();

    let mut numbers = Vec::new();

    for (start_i, op) in operations {
        // Collect column "vertical numbers" starting at start_i
        let mut cols: Vec<String> = Vec::new();

        for line in &data[..data.len() - 1] {
            let chars: Vec<char> = line.chars().collect();
            let mut found_char = false;
            let mut i = start_i;

            while i < chars.len() {
                let c = chars[i];
                let col_idx = i - start_i;

                if c == ' ' && found_char {
                    break;
                }

                if cols.len() <= col_idx {
                    cols.push(String::new());
                }

                if c != ' ' {
                    found_char = true;
                    cols[col_idx].push(c);
                }

                i += 1;
            }
        }

        // Evaluate left-to-right using op
        let mut result = None;
        for s in cols {
            let n = s.parse::<usize>().unwrap_or(0); // safe
            result = Some(match result {
                None => n,
                Some(acc) => match op {
                    '*' => acc * n,
                    '+' => acc + n,
                    _ => unreachable!(),
                },
            });
        }

        numbers.push(result.unwrap_or(0));
    }

    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 4277556);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 3263827);
    }
}
