use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    fs,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<Vec<char>>;
type Coord = (i8, i8);
type Direction = (i8, i8);

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    input
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect()
}

fn print_board(data: &InputType, visited: &HashSet<Coord>) {
    println!();

    let mut count = 0;
    data.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, col)| {
            if visited.contains(&(y as i8, x as i8)) {
                print!("#");
                count += 1;
            } else {
                print!("{}", col);
            }
        });
        println!();
    });

    println!("Count: {}", count);
}

fn resolve(start: &Coord, direction: &Direction, data: &InputType) -> usize {
    let mut queue = Vec::new();
    queue.push((*start, *direction));

    let mut seen: HashSet<(Coord, Direction)> = HashSet::new();

    while let Some(((y, x), (dy, dx))) = queue.pop() {
        if seen.contains(&((y, x), (dy, dx))) {
            continue;
        } else {
            seen.insert(((y, x), (dy, dx)));
        }

        let Some(char) = data.get(y as usize).and_then(|row| row.get(x as usize)) else {
            continue;
        };

        match (*char, (dy, dx)) {
            ('.', _) => queue.push(((y + dy, x + dx), (dy, dx))),
            ('|', (_, 0)) => queue.push(((y + dy, x + dx), (dy, dx))),
            ('-', (0, _)) => queue.push(((y + dy, x + dx), (dy, dx))),
            ('|', (0, _)) => queue.extend([((y + 1, x), (1, 0)), ((y - 1, x), (-1, 0))]),
            ('-', (_, 0)) => queue.extend([((y, x + 1), (0, 1)), ((y, x - 1), (0, -1))]),
            ('/', _) => queue.push(((y - dx, x - dy), (-dx, -dy))),
            ('\\', _) => queue.push(((y + dx, x + dy), (dx, dy))),
            _ => panic!(),
        }
    }

    seen.iter()
        .filter(|((y, x), _)| {
            *y >= 0 && *x >= 0 && *y < data.len() as i8 && *x < data[*y as usize].len() as i8
        })
        .map(|((y, x), _)| (*y, *x))
        .collect::<HashSet<(i8, i8)>>()
        .len()
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    resolve(&(0, 0), &(0, 1), &data)
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let height = data.len() as i8;
    let width = data[0].len() as i8;

    let top: Vec<(Coord, Direction)> = (0..width).map(|x| ((0, x), (1, 0))).collect();
    let bottom: Vec<(Coord, Direction)> = (0..width).map(|x| ((height - 1, x), (-1, 0))).collect();
    let left: Vec<(Coord, Direction)> = (0..height).map(|y| ((y, 0), (0, 1))).collect();
    let right: Vec<(Coord, Direction)> = (0..height).map(|y| ((y, width - 1), (0, -1))).collect();

    top.iter()
        .chain(bottom.iter())
        .chain(left.iter())
        .chain(right.iter())
        .map(|(coord, direction)| resolve(coord, direction, &data))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 51);
    }
}
