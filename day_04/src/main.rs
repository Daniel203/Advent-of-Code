use std::fs;

static TEST_INPUT: &str = "src/inputs/test-input.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_1();
    let res2 = part_2();

    println!("1: {res1}");
    println!("2: {res2}");
}

fn parse_line(line: &str) -> Vec<i32> {
    let parts = line.split(&[',', '-']).collect::<Vec<&str>>();

    return parts
        .iter()
        .map(|val| i32::from_str_radix(val, 10).unwrap())
        .collect::<Vec<i32>>();
}

fn part_1() -> i32 {
    let contents = fs::read_to_string(INPUT).unwrap();

    return contents
        .lines()
        .map(|line| parse_line(line))
        .filter(|values| {
            (values[0] <= values[2] && values[1] >= values[3])
                || (values[0] >= values[2] && values[1] <= values[3])
        })
        .count() as i32;
}

fn part_2() -> i32 {
    let contents = fs::read_to_string(INPUT).unwrap();

    return contents
        .lines()
        .map(|line| parse_line(line))
        .filter(|values| !(values[1] < values[2] || values[3] < values[0]))
        .count() as i32;
}
