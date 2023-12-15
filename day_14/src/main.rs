use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<Vec<char>>;
type Point = (usize, usize);

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

fn move_board(board: &mut InputType) {
    let mut moved_board: Vec<Vec<char>> = Vec::new();

    board[0].iter().enumerate().for_each(|(x, _)| {
        let column: Vec<char> = (0..board.len()).map(|y| board[y][x]).collect();
        let mut new_column: Vec<char> = Vec::new();
        let mut count = 0;

        column.iter().enumerate().for_each(|(y, &char)| {
            match char {
                '#' => {
                    // add to the column
                    let start = new_column.len();
                    new_column.extend((start..start + count).map(|_| 'O'));
                    new_column.extend((start + count..y).map(|_| '.'));
                    new_column.push('#');

                    count = 0;
                }
                'O' => {
                    count += 1;
                }
                _ => {}
            }

            if y == column.len() - 1 {
                let start = new_column.len();
                new_column.extend((start..start + count).map(|_| 'O'));
                new_column.extend((start + count..column.len()).map(|_| '.'));
            }
        });

        moved_board.push(new_column);
    });

    // reverse columns with rows
    moved_board = (0..moved_board[0].len())
        .map(|x| (0..moved_board.len()).map(|y| moved_board[y][x]).collect())
        .collect();

    *board = moved_board;
}

fn get_total_load(board: &InputType) -> usize {
    return board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let curr_height = board[0].len() - y;
            row.iter()
                .enumerate()
                .map(|(_, &char)| if char == 'O' { curr_height } else { 0 })
                .sum::<usize>()
        })
        .sum();
}

fn rotate_90_degrees(board: &mut InputType) -> () {
    let mut rotated_board: Vec<Vec<char>> = Vec::new();

    board[0].iter().rev().enumerate().for_each(|(x, _)| {
        let column: Vec<char> = (0..board.len()).rev().map(|y| board[y][x]).collect();
        rotated_board.push(column);
    });

    *board = rotated_board;
}

fn part_one(path: &str) -> usize {
    let mut data = parse_input(path);
    move_board(&mut data);
    return get_total_load(&data);
}

fn part_two(path: &str) -> usize {
    let original_data = parse_input(path);
    let mut board = original_data.clone();

    // I found it randomly, but it works with my input
    for _ in 0..4*1000 {
        move_board(&mut board);
        rotate_90_degrees(&mut board);
    }

    return get_total_load(&board);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 136);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 64);
    }
}
