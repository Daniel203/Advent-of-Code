use std::fs;

static INPUT: &str = "src/inputs/input.txt";

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Default for Race {
    fn default() -> Self {
        Self {time: 0, distance: 0}
    }
}

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> Vec<Race> {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let lines: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();

    let times = lines[0]
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let distances = lines[1]
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return (0..times.len())
        .map(|i| Race {
            time: times[i],
            distance: distances[i],
        })
        .collect();
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let output = data.iter().fold(1, |acc, race| {
        let count = (0..race.time)
            .filter(|i| race.distance < (race.time - i) * i)
            .count();

        return count * acc;
    });

    return output;
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let race: Race = data.iter().fold(Race::default(), |acc, race| {
        let time = format!("{}{}", acc.time, race.time)
            .parse::<usize>()
            .unwrap();
        let distance = format!("{}{}", acc.distance, race.distance)
            .parse::<usize>()
            .unwrap();

        return Race { time, distance };
    });

    let output = (0..race.time)
        .filter(|i| race.distance < (race.time - i) * i)
        .count();

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 288);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 71503);
    }
}
