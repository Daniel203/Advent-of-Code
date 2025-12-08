use std::{
    collections::HashSet,
    fs,
    str::FromStr,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<Point>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<usize> = s
            .split(',')
            .map(|v| v.trim().parse::<usize>().map_err(|_| ParsePointError))
            .collect::<Result<_, _>>()?;

        if vals.len() != 3 {
            return Err(ParsePointError);
        }

        Ok(Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        })
    }
}

fn main() {
    let res1 = part_one(INPUT_DATA, 1000);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .lines()
        .filter_map(|line| Point::from_str(line).ok())
        .collect();
    parsed_input
}

fn part_one(path: &str, n_connections: usize) -> usize {
    let points = parse_input(path);
    let mut distances: Vec<Vec<(&Point, usize)>> = Vec::new();
    let mut circuits: Vec<HashSet<Point>> = Vec::new();

    for p1 in &points {
        let mut point_distances: Vec<(&Point, usize)> = points
            .iter()
            .map(|p2| (p2, distance(p1, p2)))
            .filter(|(_, d)| *d != 0)
            .collect();
        point_distances.sort_by_key(|(_, d)| *d);
        distances.push(point_distances);
    }

    for _ in 0..n_connections {
        // Take the best connection for every point and link only the best best_conncetion
        let mut best_connection: (&Point, usize) = (&points[0], usize::MAX);
        let mut best_point_idx: usize = 0;

        for (point_idx, point_distances) in distances.iter().enumerate() {
            if point_distances.is_empty() {
                continue;
            }

            let best_point_connection = point_distances[0];
            if best_point_connection.1 < best_connection.1 {
                best_connection = best_point_connection;
                best_point_idx = point_idx;
            }
        }

        merge_points(&mut circuits, &points[best_point_idx], best_connection.0);

        // Remove connection p1->p2
        let distance_i = distances[best_point_idx]
            .iter()
            .position(|(p, _)| p == &best_connection.0);

        if let Some(distance_i) = distance_i {
            distances[best_point_idx].remove(distance_i);
        }

        // Remove connection p2->p1
        let p2_i = points.iter().position(|p| best_connection.0 == p);
        if let Some(p2_i) = p2_i {
            let p1_i = distances[p2_i]
                .iter()
                .position(|p| points[best_point_idx] == *p.0);
            if let Some(p1_i) = p1_i {
                distances[p2_i].remove(p1_i);
            }
        }
    }

    // Compute the product of the largest 3 islands
    let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a)); 
    if sizes.len() < 3 {
        0
    } else {
        sizes[0] * sizes[1] * sizes[2]
    }
}

fn part_two(path: &str) -> usize {
    let points = parse_input(path);
    let mut distances: Vec<Vec<(&Point, usize)>> = Vec::new();
    let mut circuits: Vec<HashSet<Point>> = Vec::new();

    for p1 in &points {
        let mut point_distances: Vec<(&Point, usize)> = points
            .iter()
            .map(|p2| (p2, distance(p1, p2)))
            .filter(|(_, d)| *d != 0)
            .collect();
        point_distances.sort_by_key(|(_, d)| *d);
        distances.push(point_distances);
    }

    loop {
        // Take the best connection for every point and link only the best best_conncetion
        let mut best_connection: (&Point, usize) = (&points[0], usize::MAX);
        let mut best_point_idx: usize = 0;

        for (point_idx, point_distances) in distances.iter().enumerate() {
            if point_distances.is_empty() {
                continue;
            }

            let best_point_connection = point_distances[0];
            if best_point_connection.1 < best_connection.1 {
                best_connection = best_point_connection;
                best_point_idx = point_idx;
            }
        }

        merge_points(&mut circuits, &points[best_point_idx], best_connection.0);

        if circuits.len() == 1 && circuits[0].len() == points.len() {
            return points[best_point_idx].x *  best_connection.0.x;
        }

        // Remove connection p1->p2
        let distance_i = distances[best_point_idx]
            .iter()
            .position(|(p, _)| p == &best_connection.0);

        if let Some(distance_i) = distance_i {
            distances[best_point_idx].remove(distance_i);
        }

        // Remove connection p2->p1
        let p2_i = points.iter().position(|p| best_connection.0 == p);
        if let Some(p2_i) = p2_i {
            let p1_i = distances[p2_i]
                .iter()
                .position(|p| points[best_point_idx] == *p.0);
            if let Some(p1_i) = p1_i {
                distances[p2_i].remove(p1_i);
            }
        }
    }
}

fn distance(a: &Point, b: &Point) -> usize {
    let dx = b.x as isize - a.x as isize;
    let dy = b.y as isize - a.y as isize;
    let dz = b.z as isize - a.z as isize;

    (dx * dx + dy * dy + dz * dz) as usize
}

fn merge_points(circuits: &mut Vec<HashSet<Point>>, a: &Point, b: &Point) {
    // Find existing sets containing a or b
    let mut a_idx = None;
    let mut b_idx = None;

    for (i, set) in circuits.iter().enumerate() {
        if set.contains(a) {
            a_idx = Some(i);
        }
        if set.contains(b) {
            b_idx = Some(i);
        }
    }

    match (a_idx, b_idx) {
        (None, None) => {
            // Both isolated → create new set
            let mut new_set = HashSet::new();
            new_set.insert(a.clone());
            new_set.insert(b.clone());
            circuits.push(new_set);
        }
        (Some(i), None) => {
            // a-set exists, add b
            circuits[i].insert(b.clone());
        }
        (None, Some(i)) => {
            // b-set exists, add a
            circuits[i].insert(a.clone());
        }
        (Some(i), Some(j)) if i != j => {
            // Both in different sets -> merge
            let (first, second) = if i < j { (i, j) } else { (j, i) };
            let other = circuits.remove(second);
            circuits[first].extend(other);
        }
        _ => {
            // Already in the same set → nothing to do
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA, 10), 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 25272);
    }
}
