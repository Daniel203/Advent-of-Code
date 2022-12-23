use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::{thread, time};

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

const MIN_X: usize = 100;
const MAX_X: usize = 900;
const MIN_Y: usize = 0;
const MAX_Y: usize = 200;
const DROPPING_POINT: usize = 500;

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Air,
    Sand,
    Rock,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellType::Air => write!(f, "."),
            CellType::Sand => write!(f, "o"),
            CellType::Rock => write!(f, "#"),
        }
    }
}

fn parse_input_line(line: &str) -> Vec<(usize, usize)> {
    let points: Vec<(usize, usize)> = line
        .split("->")
        .map(|p_str| {
            let coords = p_str.trim().split_once(",").unwrap();
            let x = coords.0.parse().unwrap();
            let y = coords.1.parse().unwrap();
            return (x, y);
        })
        .collect();

    return points;
}

fn generate_line_from_points(points: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut line: HashSet<(usize, usize)> = HashSet::new();

    // for every point generate the missing points to comlete the line
    for i in 0..points.len() - 1 {
        let curr = points[i];
        let next = points[i + 1];

        let mut range_x = vec![curr.0, next.0];
        range_x.sort();
        let mut range_y = vec![curr.1, next.1];
        range_y.sort();

        if curr.0 != next.0 {
            for x in range_x[0]..=range_x[1] {
                line.insert((x, curr.1.clone()));
            }
        } else {
            for y in range_y[0]..=range_y[1] {
                line.insert((curr.0.clone(), y));
            }
        }
    }

    return line.into_iter().collect();
}

fn parse_input(path: &str) -> Vec<(usize, usize)> {
    let data = fs::read_to_string(path).unwrap();
    let out: Vec<(usize, usize)> = data
        .lines()
        .map(|line| {
            // parse the string and get the points coordinates for every line
            let points_ranges = parse_input_line(line);

            // generate the missing points given the ranges to comlpete the lines
            return generate_line_from_points(points_ranges);
        })
        .into_iter()
        .flatten()
        .collect();

    return out;
}

fn generate_board(data: &Vec<(usize, usize)>) -> Vec<Vec<CellType>> {
    let mut board: Vec<Vec<CellType>> = Vec::new();

    for _ in 0..MAX_Y - MIN_Y {
        let mut row = Vec::new();
        for _ in 0..MAX_X - MIN_X {
            row.push(CellType::Air);
        }
        board.push(row);
    }

    data.iter().for_each(|point| {
        let x = point.0 - MIN_X;
        let y = point.1 - MIN_Y;
        board[y][x] = CellType::Rock;
    });

    return board;
}

fn print_board(board: &Vec<Vec<CellType>>) {
    print!("{}[2J", 27 as char);
    for row in board {
        for el in row {
            print!("{el}");
        }
        println!();
    }
    println!();

    thread::sleep(time::Duration::from_millis(100));
}

fn is_game_end(
    coords: &(usize, usize),
    board: &Vec<Vec<CellType>>,
    bottom_rocks_level: usize,
) -> bool {
    return coords.1 >= bottom_rocks_level || (coords.1 == 0 && can_move(coords, board) == None);
}

fn can_move(coords: &(usize, usize), board: &Vec<Vec<CellType>>) -> Option<(isize, isize)> {
    // check if can move down
    if coords.1 + 1 < MAX_Y && board[coords.1 + 1][coords.0] == CellType::Air {
        return Some((0, 1));
    }

    // check if can move down-left
    if coords.1 + 1 < MAX_Y
        && coords.0 - 1 > 0
        && board[coords.1 + 1][coords.0 - 1] == CellType::Air
    {
        return Some((-1, 1));
    }

    // check if can move down-right
    if coords.1 + 1 < MAX_Y
        && coords.0 + 1 < MAX_X - MIN_X
        && board[coords.1 + 1][coords.0 + 1] == CellType::Air
    {
        return Some((1, 1));
    }

    return None;
}

fn find_lowest_level_rock(board: &mut Vec<Vec<CellType>>) -> usize {
    let mut bottom_rocks_level: usize = 0;

    for (y, row) in board.iter().enumerate() {
        for el in row.iter() {
            if el == &CellType::Rock {
                bottom_rocks_level = cmp::max(bottom_rocks_level, y);
            }
        }
    }

    return bottom_rocks_level;
}

fn play_game(board: &mut Vec<Vec<CellType>>) -> usize {
    let mut count_sands = 0;

    let bottom_rocks_level = find_lowest_level_rock(board);

    loop {
        let mut coords = (DROPPING_POINT - MIN_X, 0);

        while let Some(moving_direction) = can_move(&coords, board) {
            board[coords.1][coords.0] = CellType::Air;
            coords = (
                (coords.0 as isize + moving_direction.0) as usize,
                (coords.1 as isize + moving_direction.1) as usize,
            );
            board[coords.1][coords.0] = CellType::Sand;
        }

        // place the last sand
        if coords.1 == 0 && can_move(&coords, board) == None {
            board[coords.1][coords.0] = CellType::Sand;
            count_sands += 1;
        }

        if is_game_end(&coords, &board, bottom_rocks_level) {
            break;
        }

        count_sands += 1;
    }

    return count_sands;
}

fn add_part_2_rocks(board: &mut Vec<Vec<CellType>>) {
    let y = find_lowest_level_rock(board) + 2;

    for x in 0..MAX_X - MIN_X {
        board[y][x] = CellType::Rock;
    }
}

fn part_one(path: &str) -> usize {
    let input = parse_input(path);
    let mut board = generate_board(&input);
    let result = play_game(&mut board);

    return result;
}

fn part_two(path: &str) -> usize {
    let input = parse_input(path);
    let mut board = generate_board(&input);
    add_part_2_rocks(&mut board);
    let result = play_game(&mut board);

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 24);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 93);
    }
}
