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
        .filter(|&x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    parsed_input
}

fn count_neighbors(data: &[Vec<char>], x: usize, y: usize, dirs: &[(isize, isize)]) -> usize {
    dirs.iter()
        .filter_map(|(dy, dx)| {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny >= 0 && ny < data.len() as isize && nx >= 0 && nx < data[0].len() as isize {
                Some((ny as usize, nx as usize))
            } else {
                None
            }
        })
        .filter(|&(ny, nx)| data[ny][nx] == '@')
        .count()
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let mut res = 0;

    let dirs: &[(isize, isize)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] != '@' {
                continue;
            }

            if count_neighbors(&data, x, y, dirs) < 4 {
                res += 1
            }
        }
    }

    res
}

fn part_two(path: &str) -> usize {
    let mut data = parse_input(path);

    let dirs: &[(isize, isize)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let mut temp_input = data.clone();
    let mut total_changes = 0;

    loop {
        let mut changes_count = 0;

        for y in 0..data.len() {
            for x in 0..data[0].len() {
                if data[y][x] != '@' {
                    continue;
                }

                if count_neighbors(&data, x, y, dirs) < 4 {
                    changes_count += 1;
                    temp_input[y][x] = '.';
                }
            }
        }

        if changes_count == 0 {
            return total_changes;
        }

        total_changes += changes_count;
        data = temp_input.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 43);
    }
}
