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

    let parsed_input = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();

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

// fn replace_start(
//     board: &mut [Vec<char>],
//     loop_coordinates: &[Coordinates],
//     start: Coordinates,
// ) -> () {
//     if loop_coordinates.len() < 2 {
//         return;
//     }
//
//     let next = loop_coordinates[1];
//     let prev = loop_coordinates[2];
//
//     let dy = next.0 as isize - prev.0 as isize;
//     let dx = next.1 as isize - prev.1 as isize;
//
//     let new_char = match (dy, dx) {
//         (1, 1) => {
//             if "|F7".contains(board[prev.0][prev.1]) {
//                 'L'
//             } else {
//                 'F'
//             }
//         }
//         (-1, 1) | (1, -1) => {
//             if "".contains(board[prev.0][prev.1]) {
//                 '7'
//             } else {
//                 'J'
//             }
//         }
//         // (1, 1) => 'L',
//         // (-1, 1) => 'F',
//         (-2, 0) | (2, 0) => '|',
//         (0, -2) | (0, 2) => '-',
//         _ => panic!("unexpected coordinates"),
//     };
//
//     board[start.0][start.1] = new_char;
//
//     return ();
// }

fn part_one(path: &str) -> usize {
    let board = parse_input(path);
    return get_path(find_start(&board), &board).len() / 2;
}

fn part_two(path: &str) -> usize {
    let mut board = parse_input(path);
    let start = find_start(&board);
    let loop_coordinates = get_path(start, &board.clone());

    // replace outside characters with '.'
    board.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, col)| {
            if !loop_coordinates.contains(&(y, x)) {
                *col = '.';
            }
        });
    });

    // board[start.0][start.1] = 'L';
    board[start.0][start.1] = 'F';
    // replace_start(&mut board, &loop_coordinates, start);

    let mut outside = HashSet::new();
    let mut within = false;
    let mut up = false;

    // search intersections in horizontal direction
    for (y, row) in board.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            match col {
                '|' => within = !within,
                'L' | 'F' => up = col == 'L',
                '7' | 'J' => {
                    let expected_char = if up { 'J' } else { '7' };

                    if col != expected_char {
                        within = !within;
                    }

                    up = false;
                }
                '.' | '-' => { /* do nothing */ }
                _ => panic!("unexpected character (horizontal): {}", col),
            }

            // keey track of the coordinates outside the loop
            if !within {
                outside.insert((y, x));
            }
        }
    }

    // return the number of elementes in the board minus the number of elements outside the loop
    // we add the loop to the outside set
    outside.extend(loop_coordinates.clone());
    return (board.len() * board[0].len()) - outside.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";
    static INPUT_TEST_DATA_2: &str = "src/inputs/test_input_2.txt";
    static INPUT_TEST_DATA_3: &str = "src/inputs/test_input_3.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 8);
    }

    #[test]
    fn test_part_two_1() {
        assert_eq!(part_two(INPUT_TEST_DATA), 0);
    }

    #[test]
    fn test_part_two_2() {
        assert_eq!(part_two(INPUT_TEST_DATA_2), 4);
    }

    #[test]
    fn test_part_two_3() {
        assert_eq!(part_two(INPUT_TEST_DATA_3), 8);
    }
}
