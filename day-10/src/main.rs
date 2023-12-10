use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type Coordinates = (usize, usize);
type Board = Vec<Vec<char>>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> Board {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input.split("\n").map(|x| x.chars().collect()).collect();
    return parsed_input;
}

fn find_start(data: &Board) -> Coordinates {
    let mut start: Coordinates = (0, 0);
    data.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &col)| {
            if col == 'S' {
                start = (y, x);
            }
        });
    });
    return start;
}

fn get_path(start: Coordinates, board: &Board) -> Vec<Coordinates> {
    let mut loop_set: Vec<Coordinates> = Vec::new();
    let mut queue: VecDeque<Coordinates> = VecDeque::new();

    loop_set.push(start);
    queue.push_back(start);

    while let Some((y, x)) = queue.pop_front() {
        let ch = board[y][x];

        // north
        if y > 0
            && "S|JL".contains(ch)
            && "|7F".contains(board[y - 1][x])
            && !loop_set.contains(&(y - 1, x))
        {
            loop_set.push((y - 1, x));
            queue.push_back((y - 1, x));
        }

        // south
        if y < board.len() - 1
            && "S|7F".contains(ch)
            && "|JL".contains(board[y + 1][x])
            && !loop_set.contains(&(y + 1, x))
        {
            loop_set.push((y + 1, x));
            queue.push_back((y + 1, x));
        }

        // west
        if x > 0
            && "S-J7".contains(ch)
            && "-LF".contains(board[y][x - 1])
            && !loop_set.contains(&(y, x - 1))
        {
            loop_set.push((y, x - 1));
            queue.push_back((y, x - 1));
        }

        // east
        if x < board[y].len() - 1
            && "S-LF".contains(ch)
            && "-J7".contains(board[y][x + 1])
            && !loop_set.contains(&(y, x + 1))
        {
            loop_set.push((y, x + 1));
            queue.push_back((y, x + 1));
        }
    }

    return loop_set;
}

fn part_one(path: &str) -> usize {
    let board = parse_input(path);
    return get_path(find_start(&board), &board).len() / 2;
}

fn part_two(path: &str) -> usize {
    let board = parse_input(path);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 10);
    }
}
