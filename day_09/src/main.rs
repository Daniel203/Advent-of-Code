use std::{
    collections::{HashMap, HashSet},
    fs,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<(usize, usize)>;

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| {
            let (y_str, x_str) = x.split_once(",").unwrap();
            let y = y_str.parse().unwrap();
            let x = x_str.parse().unwrap();
            (y, x)
        })
        .collect();
    parsed_input
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let mut max_area = 0;
    for (i, p1) in data.iter().enumerate() {
        for p2 in data.iter().skip(i + 1) {
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let mut polygon = HashSet::new();

    for i in 0..data.len() {
        let (p1_y, p1_x) = data[(i + 1) % data.len()];
        let (p2_y, p2_x) = data[i];

        if p1_y == p2_y {
            let start = p1_x.min(p2_x);
            let end = p1_x.max(p2_x);
            for x in start..end + 1 {
                polygon.insert((p1_y, x));
            }
        } else if p1_x == p2_x {
            let start = p1_y.min(p2_y);
            let end = p1_y.max(p2_y);
            for y in start..end + 1 {
                polygon.insert((y, p1_x));
            }
        }
    }

    let data: HashSet<(usize, usize)> = parse_input(path).iter().copied().collect();

    // Build spatial index: group points by row and column
    let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut cols: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(y, x) in &data {
        rows.entry(y).or_default().push(x);
        cols.entry(x).or_default().push(y);
    }

    for v in rows.values_mut() {
        v.sort_unstable();
    }

    for v in cols.values_mut() {
        v.sort_unstable();
    }

    let points: Vec<_> = data.iter().copied().collect();

    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for &p2 in points.iter().skip(i + 1) {
            // Skip if points are on same row or column (not diagonal)
            if p1.0 == p2.0 || p1.1 == p2.1 {
                continue;
            }

            let (y1, x1) = p1;
            let (y2, x2) = p2;

            // Determine rectangle bounds
            let min_y = (*y1).min(y2);
            let max_y = (*y1).max(y2);
            let min_x = (*x1).min(x2);
            let max_x = (*x1).max(x2);

            let mut valid = true;

            // Check top edge
            if let Some(x_coords) = rows.get(&min_y) {
                for &x in x_coords {
                    if x > min_x && x < max_x {
                        valid = false;
                        break;
                    }
                }
            }

            // Check bottom edge
            if valid {
                if let Some(x_coords) = rows.get(&max_y) {
                    for &x in x_coords {
                        if x > min_x && x < max_x {
                            valid = false;
                            break;
                        }
                    }
                }
            }

            // Check left edge
            if valid {
                if let Some(y_coords) = cols.get(&min_x) {
                    for &y in y_coords {
                        if y > min_y && y < max_y {
                            valid = false;
                            break;
                        }
                    }
                }
            }

            // Check right edge
            if valid {
                if let Some(y_coords) = cols.get(&max_x) {
                    for &y in y_coords {
                        if y > min_y && y < max_y {
                            valid = false;
                            break;
                        }
                    }
                }
            }

            if valid {
                let area = (max_y - min_y + 1) * (max_x - min_x + 1);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 50);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 24);
    }
}
