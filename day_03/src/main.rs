use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let mut matrix: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
    matrix.pop();

    return matrix;
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let mut visited_values: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();

    data.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, el)| {
            if el.is_digit(10) {
                let mut num_str = el.to_string();
                let mut x_left = x;
                let mut x_right = x;

                while x_left >= 1 && row.chars().nth(x_left - 1).unwrap().is_digit(10) {
                    x_left -= 1;
                    let char = row.chars().nth(x_left).unwrap();
                    num_str = char.to_string() + &num_str;
                }

                while x_right + 1 < row.len() && row.chars().nth(x_right + 1).unwrap().is_digit(10)
                {
                    x_right += 1;
                    let char = row.chars().nth(x_right).unwrap();
                    num_str = num_str + &char.to_string();
                }

                let num = num_str.parse::<usize>().unwrap();
                let coodridantes = ((y, x_left), (y, x_right));

                if !visited_values.contains_key(&coodridantes) {
                    let range_y = max(0, y as isize - 1) as usize..=min(y + 1, data.len() - 1);
                    let range_x =
                        max(0, x_left as isize - 1) as usize..=min(x_right + 1, data[0].len() - 1);

                    (range_y.clone()).for_each(|j| {
                        (range_x.clone()).for_each(|i| {
                            let char = data[j].chars().nth(i).unwrap();
                            if !"0123456789.".contains(char) {
                                visited_values.insert(((y, x_left), (y, x_right)), num);
                            }
                        })
                    });
                }
            }
        });
    });

    let sum = visited_values.values().sum::<usize>();
    return sum;
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let mut values: Vec<usize> = Vec::new();

    data.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, el)| {
            if el == '*' {
                // check all the numbers around
                let mut visited_values: HashMap<((usize, usize), (usize, usize)), usize> =
                    HashMap::new();

                let range_y = max(0, y as isize - 1) as usize..=min(y + 1, data.len() - 1);
                let range_x = max(0, x as isize - 1) as usize..=min(x + 1, data[0].len() - 1);

                (range_y.clone()).for_each(|j| {
                    (range_x.clone()).for_each(|i| {
                        let char = data[j].chars().nth(i).unwrap();
                        if char.is_digit(10) {
                            let mut num_str = char.to_string();
                            let mut x_left = i;
                            let mut x_right = i;

                            while x_left >= 1
                                && data[j].chars().nth(x_left - 1).unwrap().is_digit(10)
                            {
                                x_left -= 1;
                                let char = data[j].chars().nth(x_left).unwrap();
                                num_str = char.to_string() + &num_str;
                            }

                            while x_right + 1 < data[j].len()
                                && data[j].chars().nth(x_right + 1).unwrap().is_digit(10)
                            {
                                x_right += 1;
                                let char = data[j].chars().nth(x_right).unwrap();
                                num_str = num_str + &char.to_string();
                            }

                            let num = num_str.parse::<usize>().unwrap();
                            visited_values.insert(((y, x_left), (y, x_right)), num);
                        }
                    })
                });

                if visited_values.values().len() >= 2 {
                    let multiplication = visited_values.values().product::<usize>();
                    values.push(multiplication);
                }
            }
        });
    });

    return values.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 467835);
    }
}
