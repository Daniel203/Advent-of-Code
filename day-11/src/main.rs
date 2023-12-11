use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = (Vec<Coord>, Vec<Vec<char>>);
type Coord = (usize, usize);
type Board = Vec<Vec<char>>;

fn main() {
    let res1 = resolve(INPUT_DATA, 1);
    let res2 = resolve(INPUT_DATA, 1_000_000-1);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let mut points: Vec<Coord> = Vec::new();

    let board = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, col)| {
                    if col == '#' {
                        points.push((y, x));
                    }
                    return col;
                })
                .collect()
        })
        .collect();

    return (points, board);
}

fn find_empty_lines(board: &Board) -> (Vec<usize>, Vec<usize>) {
    // Check rows
    let empty_rows = board
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&cell| cell == '.'))
        .map(|(y, _)| y)
        .collect::<Vec<usize>>();

    // Check columns
    let empty_cols = (0..board[0].len())
        .filter(|&x| board.iter().all(|row| row[x] == '.'))
        .collect::<Vec<usize>>();

    return (empty_rows, empty_cols);
}

fn get_all_paths_endpoints(points: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut endpoints = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            endpoints.push((points[i], points[j]));
        }
    }

    return endpoints;
}

fn add_empty_lines_to_endpoints(
    endpoints: &[(Coord, Coord)],
    empty_rows: &[usize],
    empty_cols: &[usize],
    n_added_lines: usize,
) -> Vec<(Coord, Coord)> {
    endpoints
        .iter()
        .map(|((y1, x1), (y2, x2))| {
            let add_y_1 = empty_rows.iter().filter(|&y| y <= y1).count() * n_added_lines;
            let add_x_1 = empty_cols.iter().filter(|&x| x <= x1).count() * n_added_lines;
            let add_y_2 = empty_rows.iter().filter(|&y| y <= y2).count() * n_added_lines;
            let add_x_2 = empty_cols.iter().filter(|&x| x <= x2).count() * n_added_lines;

            return (
                (*y1 + add_y_1, *x1 + add_x_1),
                (*y2 + add_y_2, *x2 + add_x_2),
            );
        })
        .collect()
}

fn resolve(path: &str, n_added_lines: usize) -> usize {
    let (points, board) = parse_input(path);
    let (empty_rows, empty_cols) = find_empty_lines(&board);

    let endpoints = get_all_paths_endpoints(&points);
    let endpoints_expandend =
        add_empty_lines_to_endpoints(&endpoints, &empty_rows, &empty_cols, n_added_lines);

    return endpoints_expandend
        .iter()
        .map(|((y1, x1), (y2, x2))| {
            return y1.abs_diff(*y2) + x1.abs_diff(*x2);
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(resolve(INPUT_TEST_DATA, 1), 374);
    }

    #[test]
    fn test_part_two_1() {
        assert_eq!(resolve(INPUT_TEST_DATA, 10-1), 1030);
    }

    #[test]
    fn test_part_two_2() {
        assert_eq!(resolve(INPUT_TEST_DATA, 100-1), 8410);
    }
}
