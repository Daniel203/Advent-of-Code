use std::fs;

static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

#[derive(Debug)]
struct Game {
    id: usize,
    red: Vec<usize>,
    green: Vec<usize>,
    blue: Vec<usize>,
}

fn parse_input(path: &str) -> Vec<Game> {
    let input = fs::read_to_string(path).expect("Error while reading the file!");

    let parsed_input: Vec<Game> = input
        .split("\n")
        .map(|line| {
            if line == "" {
                return Game {
                    id: 0,
                    red: Vec::new(),
                    green: Vec::new(),
                    blue: Vec::new(),
                };
            }

            let split_line: Vec<&str> = line.split(":").collect();
            let id = split_line[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();

            let mut red = Vec::new();
            let mut green = Vec::new();
            let mut blue = Vec::new();

            let extractions: Vec<&str> = split_line[1].split(";").collect();
            extractions.iter().for_each(|extraction| {
                let colors = extraction.split(",").collect::<Vec<&str>>();
                for color in colors {
                    let splitted: Vec<&str> = color.split(" ").collect();
                    let val_tmp = splitted[1].parse::<usize>().unwrap();
                    match splitted[2] {
                        "red" => red.push(val_tmp),
                        "green" => green.push(val_tmp),
                        "blue" => blue.push(val_tmp),
                        _ => (),
                    }
                }
            });

            Game {
                id,
                red,
                green,
                blue,
            }
        })
        .collect();

    parsed_input
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    let max_red: usize = 12;
    let max_green: usize = 13;
    let max_blue: usize = 14;

    let output = data
        .iter()
        .filter(|game| {
            game.red.iter().all(|val| val <= &max_red)
                && game.green.iter().all(|val| val <= &max_green)
                && game.blue.iter().all(|val| val <= &max_blue)
        })
        .map(|game| game.id)
        .sum();

    return output;
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let output = data
        .iter()
        .map(|game| {
            if game.red.len() == 0 || game.green.len() == 0 || game.blue.len() == 0 {
                return 0;
            }

            let max_red = game.red.iter().max().unwrap_or_else(|| &1);
            let max_green = game.green.iter().max().unwrap_or_else(|| &1);
            let max_blue = game.blue.iter().max().unwrap_or_else(|| &1);

            max_red * max_green * max_blue
        })
        .sum();

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 2286);
    }
}
