use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<(char, usize, [u8; 6])>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .lines()
        .map(|x| {
            let dir = x.chars().nth(0).unwrap();
            let (len, color) = x[2..].split_once(" ").unwrap();
            let len = len.parse::<usize>().unwrap();
            let color = color[2..8].as_bytes().try_into().unwrap();
            (dir, len, color)
        })
        .collect();
    return parsed_input;
}

fn solve(iterator: impl Iterator<Item = (char, usize)>) -> usize {
    let mut area = 0;
    let mut perimeter = 0;
    let mut y = 0;

    iterator.for_each(|(dir, len)| {
        let _len = len as isize;
        match dir {
            'U' => y -= _len,
            'D' => y += _len,
            'R' => area += _len * y,
            'L' => area -= _len * y,
            _ => panic!(),
        }
        perimeter += len;
    });

    return area.abs() as usize + perimeter / 2 + 1;
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    solve(data.iter().map(|(dir, len, _)| (*dir, *len)))
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let hex_to_dir = |hex: u8| match hex {
        b'0' => 'R',
        b'1' => 'D',
        b'2' => 'L',
        b'3' => 'U',
        _ => panic!(),
    };

    let hex_to_len = |hex: u8| match hex {
        b'0'..=b'9' => (hex - b'0') as usize,
        b'a'..=b'f' => (hex - b'a' + 10) as usize,
        _ => panic!(),
    };

    solve(data.iter().map(|(_, _, color)| {
        let dir = hex_to_dir(color[5]);
        let len = color[..5]
            .iter()
            .fold(0, |acc, &x| acc * 16 + hex_to_len(x));

        (dir, len)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 62);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 952408144115);
    }
}
