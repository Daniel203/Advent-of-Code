use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<String>;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    value: usize,
}

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .split(",")
        .map(|x| x.to_string().replace("\n", ""))
        .collect();
    return parsed_input;
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    return data
        .iter()
        .map(|str| {
            str.chars().fold(0, |curr_value, char| {
                (curr_value + char as usize) * 17 % 256
            })
        })
        .sum();
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for str in data {
        let mut box_index = 0;

        if str.contains("=") {
            let splitted_line = str.split("=").collect::<Vec<&str>>();

            let lens = Lens {
                label: splitted_line[0].to_string(),
                value: splitted_line[1].parse::<usize>().unwrap(),
            };

            for char in lens.label.chars() {
                box_index += char as usize;
                box_index *= 17;
                box_index %= 256;
            }

            // searh for the lens in the box
            let old_lens_indexes: Vec<usize> = boxes[box_index]
                .iter()
                .enumerate()
                .filter(|(_, x)| x.label == lens.label)
                .map(|(i, _)| i)
                .collect();

            if old_lens_indexes.is_empty() {
                boxes[box_index].push(lens);
            } else {
                let old_lens = old_lens_indexes[0];
                boxes[box_index][old_lens].value = lens.value;
            }
        } else {
            let splitted_line = str.split("-").collect::<Vec<&str>>();
            let label = splitted_line[0];

            for char in label.chars() {
                box_index += char as usize;
                box_index *= 17;
                box_index %= 256;
            }

            boxes[box_index] = boxes[box_index]
                .iter()
                .filter(|x| x.label != label)
                .map(|x| x.clone())
                .collect();
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, boxes)| {
            boxes.iter().enumerate().fold(0, |sum, (j, lens)| {
                let val = (i + 1) * (j + 1) * lens.value;
                return sum + val;
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 1320);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 145);
    }
}
