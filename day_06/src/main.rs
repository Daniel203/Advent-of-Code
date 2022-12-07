use std::fs;

const TEST_INPUT: &str = "src/inputs/test-input.txt";
const INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_1();
    let res2 = part_2();

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn get_start_of_packet(input: &[u8], start_of_message_size: usize) -> usize {
    let mut j = 0;

    for i in 0..input.len() {
        let mut buffer: Vec<u8> = Vec::new();

        while !buffer.contains(&input[j])
            && buffer.len() != start_of_message_size
            && j < input.len()
        {
            buffer.push(input[j]);
            j += 1
        }

        if buffer.len() == start_of_message_size {
            return j;
        }

        j = i + 1;
    }

    return 0;
}

fn part_1() -> usize {
    let input = fs::read_to_string(INPUT).expect("Error while reading the file");

    return get_start_of_packet(input.as_bytes(), 4);
}

fn part_2() -> usize {
    let input = fs::read_to_string(INPUT).expect("Error while reading the file");

    return get_start_of_packet(input.as_bytes(), 14);
}
