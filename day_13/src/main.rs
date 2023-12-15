use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type Board = Vec<Vec<char>>;
type InputType = Vec<Board>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .split("\n\n")
        .map(|board| {
            board
                .split("\n")
                .filter(|row| !row.is_empty())
                .map(|row| row.chars().collect())
                .collect()
        })
        .collect();

    return parsed_input;
}

fn find_sum_rows(board: &Board) -> usize {
    (0..board.len() - 1)
        .filter(|&y| {
            let row = &board[y];
            let next_row = &board[y + 1];

            if row.eq(next_row) {
                // check that all the other rows are equals
                let all_equals = (1..board.len() - 1)
                    .filter(|&i| i <= y && y + i + 1 < board.len())
                    .all(|i| {
                        let prev_row = &board[y - i];
                        let next_next_row = &board[y + i + 1];
                        return prev_row.eq(next_next_row);
                    });

                return all_equals;
            }

            return false;
        })
        .map(|y| y + 1)
        .sum::<usize>()
        * 100
}

fn find_sum_columns(board: &Board) -> usize {
    (0..board[0].len() - 1)
        .filter(|&x| {
            let column: Vec<char> = (0..board.len()).map(|y| board[y][x]).collect();
            let next_column: Vec<char> = (0..board.len()).map(|y| board[y][x + 1]).collect();

            if column.eq(&next_column) {
                let all_equals = (1..board[0].len() - 1)
                    .filter(|&i| i <= x && x + i + 1 < board[0].len())
                    .all(|i| {
                        let prev_column: Vec<char> =
                            (0..board.len()).map(|y| board[y][x - i]).collect();
                        let next_next_column: Vec<char> =
                            (0..board.len()).map(|y| board[y][x + i + 1]).collect();
                        return prev_column.eq(&next_next_column);
                    });

                return all_equals;
            }

            return false;
        })
        .map(|x| x + 1)
        .sum::<usize>()
}

fn fix_smudges(data: &Vec<Board>) -> Vec<Board> {
    let mut fixed_boards: Vec<Board> = Vec::new();

    for board in data {
        // check the rows 
        // board.iter()
    }

    return fixed_boards;
}

fn resolve(boards: Vec<Board>) -> usize {
    boards
        .iter()
        .map(|board| {
            // check the rows
            let sum_rows = find_sum_rows(&board);
            if sum_rows > 0 {
                println!("sum_rows: {}", sum_rows);
                return sum_rows;
            }

            // check the columns
            let sum_columns = find_sum_columns(&board);
            if sum_columns > 0 {
                return sum_columns;
            }

            return 0;
        })
        .sum()
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    resolve(data)
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let fixed_boards = fix_smudges(&data);

    resolve(fixed_boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 405);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 400);
    }
}
