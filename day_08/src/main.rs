use std::fs;

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn get_valid_trees(data: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut valid_trees: Vec<(usize, usize)> = Vec::new();

    // get the sizes of the matrix
    let column_len = data.len() as i32;
    let row_len = data[0].len() as i32;

    // perform the actual count of trees
    for i in 1..column_len - 1 {
        for j in 1..row_len - 1 {
            let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            let mut valid: Vec<bool> = Vec::new();
            let data_to_compare = data[i as usize][j as usize];

            for direction in directions {
                let mut x: i32 = j + direction.1;
                let mut y: i32 = i + direction.0;
                let mut valid_in_direction = true;

                while x < row_len && x >= 0 && y < column_len && y >= 0 && valid_in_direction {
                    let data_to_compare_with = data[y as usize][x as usize];

                    if data_to_compare <= data_to_compare_with {
                        valid_in_direction = false;
                    } else {
                        x += direction.1;
                        y += direction.0;
                    }
                }

                valid.push(valid_in_direction);
            }

            if valid.contains(&true) {
                valid_trees.push((i as usize, j as usize));
            }
        }
    }

    return valid_trees;
}

fn convert_to_matrix(input: &str) -> Vec<Vec<u8>> {
    let data: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|val| val - 48).collect())
        .collect();

    return data;
}

fn part_one(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();

    // convert input to a matrix 2x2
    let data = convert_to_matrix(&input);

    // count trees
    let mut count = 0;
    count += (data.len() - 1) * 4;
    count += get_valid_trees(&data).len();

    return count;
}

fn part_two(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();

    // convert input to a matrix 2x2
    let data = convert_to_matrix(&input);

    // get the sizes of the matrix
    let column_len = data.len() as i32;
    let row_len = data[0].len() as i32;

    // search for max viewing distance
    let mut max_viewing_distance = 1;

    for (i, j) in get_valid_trees(&data) {
        let mut current_max = 1;
        let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let data_to_compare = data[i as usize][j as usize];

        for direction in directions {
            let mut x: i32 = j as i32 + direction.1;
            let mut y: i32 = i as i32 + direction.0;
            let mut valid_in_direction = true;
            let mut current_count = 0;

            while x < row_len && x >= 0 && y < column_len && y >= 0 && valid_in_direction {
                let data_to_compare_with = data[y as usize][x as usize];
                if data_to_compare <= data_to_compare_with {
                    valid_in_direction = false;
                }
                current_count += 1;
                x += direction.1;
                y += direction.0;
            }

            current_max *= current_count;
        }

        if current_max > max_viewing_distance {
            max_viewing_distance = current_max;
        }
    }

    return max_viewing_distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 8);
    }
}
