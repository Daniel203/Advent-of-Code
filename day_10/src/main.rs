use std::fs;

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = (usize, Vec<usize>, Vec<usize>);

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_pattern_bits(s: &str) -> usize {
    let mut bits = 0;
    let s = s.trim_matches(['[', ']']);
    for (i, c) in s.chars().enumerate() {
        if c == '#' {
            bits |= 1 << i;
        }
    }
    bits
}

fn parse_tuple_bits(s: &str) -> usize {
    let inside = s.trim_matches(['(', ')']);
    let mut mask = 0;

    if !inside.is_empty() {
        for n in inside.split(',') {
            let v: usize = n.parse().unwrap();
            mask |= 1 << v;
        }
    }
    mask
}

fn parse_brace_bits(s: &str) -> Vec<usize> {
    let inside = s.trim_matches(['{', '}']);
    let mut joltages = Vec::new();

    if !inside.is_empty() {
        for n in inside.split(',') {
            let j: usize = n.parse().unwrap();
            joltages.push(j);
        }
    }
    joltages
}

fn parse_line(line: &str) -> InputType {
    let mut parts = line.split_whitespace();

    // 1. pattern
    let pattern_str = parts.next().unwrap();
    let pattern = parse_pattern_bits(pattern_str);

    // 2. tuple masks
    let mut tuple_masks = Vec::new();
    let mut joltages = Vec::new();

    for p in parts {
        if p.starts_with('(') {
            tuple_masks.push(parse_tuple_bits(p));
        } else if p.starts_with('{') {
            joltages = parse_brace_bits(p);
        }
    }

    (pattern, tuple_masks, joltages)
}

fn parse_input(path: &str) -> Vec<InputType> {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(parse_line)
        .collect();
    parsed_input
}

fn solve_1(pattern: usize, operations: &Vec<usize>) -> usize {
    use std::collections::{HashMap, VecDeque};

    let mut visited: HashMap<usize, usize> = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((0, 0)); // (state, steps)

    while let Some((state, steps)) = q.pop_front() {
        if visited.contains_key(&state) {
            continue;
        }
        visited.insert(state, steps);

        if state == pattern {
            return steps;
        }

        for &op in operations {
            q.push_back((state ^ op, steps + 1));
        }
    }

    panic!("unreachable");
}

fn solve_2(operations: &[Vec<usize>], joltage: &[usize]) -> usize {
    use z3::ast::Ast; 
    use z3::{ast::Int, Config, Context, Optimize};

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    // Each button gets an integer variable representing how many times it's pressed
    let button_vars: Vec<Int> = operations
        .iter()
        .enumerate()
        .map(|(i, _)| Int::new_const(&ctx, format!("b{}", i)))
        .collect();

    // Add constraints: each button press >= 0
    for var in &button_vars {
        opt.assert(&var.ge(&Int::from_i64(&ctx, 0)));
    }

    // For each counter, ensure that the sum of button presses that affect it equals the target joltage
    for (counter_idx, &target) in joltage.iter().enumerate() {
        let mut sum = Int::from_i64(&ctx, 0);

        for (button_idx, button) in operations.iter().enumerate() {
            if button.contains(&counter_idx) {
                sum += &button_vars[button_idx];
            }
        }

        opt.assert(&sum._eq(&Int::from_i64(&ctx, target as i64)));
    }

    // Objective: minimize total presses
    let total_presses = button_vars
        .iter()
        .fold(Int::from_i64(&ctx, 0), |acc, v| acc + v);
    opt.minimize(&total_presses);

    // Solve
    let model = opt.check(&[]);
    assert!(model == z3::SatResult::Sat, "No solution found!");
    let model = opt.get_model().unwrap();

    let mut total = 0;
    for var in &button_vars {
        let val = model.eval(var, true).unwrap().as_i64().unwrap();
        total += val as usize;
    }

    total
}

fn mask_to_indices(mask: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    for i in 0..64 {
        // assuming usize ≤ 64 bits
        if (mask & (1 << i)) != 0 {
            indices.push(i);
        }
    }
    indices
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    data.iter()
        .map(|(pattern, operations, _joltage)| solve_1(*pattern, operations))
        .sum()
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    data.iter()
        .map(|(_pattern, operations, joltage)| {
            let operations: Vec<Vec<usize>> = operations
                .iter()
                .map(|&mask| mask_to_indices(mask))
                .collect();

            solve_2(&operations, joltage)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 7);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 33);
    }
}
