use std::fs;

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

fn main() {
    let res1 = part_one(INPUT);
    let res2 = part_two(INPUT);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

#[derive(Debug, Default)]
struct Monkey<'a> {
    items: Vec<u64>,
    operation: &'a str,
    operation_value: &'a str, // because it can be "old"
    divisible_by: u64,
    throw_if_true: usize,  // use the indexes of the monkey
    throw_if_false: usize, // use the indexes of the monkey
}

impl Monkey<'_> {
    fn new() -> Self {
        return Default::default();
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    return input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|monkey_str| {
            let monkey_vec: Vec<&str> = monkey_str.split("\n").collect();

            let mut monkey = Monkey::new();

            // read the items
            let items_str = monkey_vec[1].split_once(":").unwrap().1;
            monkey.items = items_str
                .split(",")
                .map(|el| el.trim().parse().unwrap())
                .collect();

            // parse the operation
            let operation_and_value_str = monkey_vec[2].split_once("=").unwrap().1;
            (monkey.operation, monkey.operation_value) = operation_and_value_str
                .split_once("old")
                .unwrap()
                .1
                .trim()
                .split_once(" ")
                .unwrap();

            // parse he divisible_by
            monkey.divisible_by = monkey_vec[3].rsplit_once(" ").unwrap().1.parse().unwrap();

            // parse throw if true and throw if false
            monkey.throw_if_true = monkey_vec[4].rsplit_once(" ").unwrap().1.parse().unwrap();
            monkey.throw_if_false = monkey_vec[5].rsplit_once(" ").unwrap().1.parse().unwrap();

            return monkey;
        })
        .collect();
}

fn do_operation(worry_level: u64, operation: &str, operation_value_str: &str) -> u64 {
    let operation_value: u64 = match operation_value_str {
        "old" => worry_level,
        _ => operation_value_str.parse().unwrap(),
    };

    return match operation {
        "*" => worry_level * operation_value,
        "+" => worry_level + operation_value,
        _ => worry_level,
    };
}

fn process(path: &str, n_cicles: u64, part_2: bool) -> u64 {
    let input = fs::read_to_string(path).expect("Error while reading the file");
    let mut monkeys = parse_input(&input);
    let mut counts: Vec<u64> = vec![0; monkeys.len()];

    let modulo: u64 = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    for _ in 0..n_cicles {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                counts[i] += 1;
                let mut worry_level = do_operation(
                    monkeys[i].items[0],
                    monkeys[i].operation,
                    monkeys[i].operation_value,
                ) ;

                if part_2 {
                    worry_level = worry_level % modulo;
                } else {
                    worry_level = worry_level / 3;
                }

                monkeys[i].items.remove(0);

                let remainder = worry_level % monkeys[i].divisible_by;
                if remainder == 0 {
                    let throw_if_true = monkeys[i].throw_if_true;
                    monkeys[throw_if_true].items.push(worry_level);
                } else {
                    let throw_if_false = monkeys[i].throw_if_false;
                    monkeys[throw_if_false].items.push(worry_level);
                }
            }
        }
    }

    counts.sort_by(|a, b| b.partial_cmp(a).unwrap());

    return counts[0] * counts[1]; 
}

fn part_one(path: &str) -> u64 {
    return process(path ,  20, false);
}

fn part_two(path: &str) -> u64 {
    return process(path, 10000, true);
}

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 10605);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 2713310158);
    }
}
