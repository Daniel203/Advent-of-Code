use std::collections::{HashSet, VecDeque};
use std::fs;

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> Vec<Vec<u8>> {
    let input = fs::read_to_string(path).expect("Error while reading the file");

    let mut output: Vec<Vec<u8>> = Vec::new();
    input.split("\n").for_each(|line| {
        if !line.is_empty() {
            output.push(line.as_bytes().to_vec());
        }
    });

    return output;
}

fn get_start_and_end(board: &Vec<Vec<u8>>) -> ((isize, isize), (isize, isize)) {
    let mut start: (isize, isize) = (0, 0);
    let mut end: (isize, isize) = (0, 0);

    for (y, row) in board.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == b'S' {
                start = (y as isize, x as isize);
            } else if *val == b'E' {
                end = (y as isize, x as isize);
            }
        }
    }

    return (start, end);
}

fn get_neighbours_forward(board: &Vec<Vec<u8>>, row: isize, col: isize) -> Vec<(isize, isize)> {
    let height = board.len() as isize;
    let width = board[0].len() as isize;

    let max_el = board[row as usize][col as usize] + 1;

    let directions: Vec<(isize, isize)> = vec![
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ];
    let mut out: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_row, n_col) in directions {
        if n_row >= 0 && n_row < height && n_col >= 0 && n_col < width {
            if board[n_row as usize][n_col as usize] <= max_el {
                out.push((n_row, n_col));
            }
        }
    }

    return out;
}

fn get_neighbours_backward(board: &Vec<Vec<u8>>, row: isize, col: isize) -> Vec<(isize, isize)> {
    let height = board.len() as isize;
    let width = board[0].len() as isize;

    let min_el = board[row as usize][col as usize] - 1;

    let directions: Vec<(isize, isize)> = vec![
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ];
    let mut out: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_row, n_col) in directions {
        if n_row >= 0 && n_row < height && n_col >= 0 && n_col < width {
            if board[n_row as usize][n_col as usize] >= min_el {
                out.push((n_row, n_col));
            }
        }
    }

    return out;
}

fn bfs(
    board: &Vec<Vec<u8>>,
    start: (isize, isize),
    end: (isize, isize),
    end_val: u8,
    get_neighbours: &dyn Fn(&Vec<Vec<u8>>, isize, isize) -> Vec<(isize, isize)>,
) -> i32 {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue = VecDeque::from([(0, start)]);

    while queue.len() != 0 {
        let (distance, coords) = queue.pop_front().unwrap();

        if coords == end || board[coords.0 as usize][coords.1 as usize] == end_val {
            return distance;
        }

        if visited.contains(&coords) == false {
            visited.insert(coords);

            for neighbour in get_neighbours(&board, coords.0, coords.1) {
                if visited.contains(&neighbour) == false {
                    queue.push_back((distance + 1, neighbour));
                }
            }
        }
    }

    return -1;
}

fn part_one(path: &str) -> i32 {
    let mut board = parse_input(path);
    let (start, end) = get_start_and_end(&board);

    board[start.0 as usize][start.1 as usize] = b'a';
    board[end.0 as usize][end.1 as usize] = b'z';

    return bfs(&board, start, end, 0, &get_neighbours_forward);
}

fn part_two(path: &str) -> i32 {
    let mut board = parse_input(path);
    let (start, end) = get_start_and_end(&board);

    board[start.0 as usize][start.1 as usize] = b'a';
    board[end.0 as usize][end.1 as usize] = b'z';

    return bfs(&board, end, (-1, -1), b'a', &get_neighbours_backward);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 31);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 29);
    }
}
