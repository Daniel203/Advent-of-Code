use std::fs;

static TEST_INPUT: &str = "src/inputs/test_input.txt";
static INPUT: &str = "src/inputs/input.txt";

type Point = (isize, isize);

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let res1 = part_one(INPUT, 2000000);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let res2 = part_two(INPUT);
    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> (Vec<Point>, Vec<Point>) {
    let data = fs::read_to_string(path).unwrap();
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    data.lines()
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .for_each(|el| {
            let sensor_str = el[0];
            let beacon_str = el[1];

            // parse sensor
            let (sensor_x_str, sensor_y_str) = sensor_str
                .split_once("at ")
                .unwrap()
                .1
                .trim()
                .split_once(", ")
                .unwrap();
            let sensor_x: isize = sensor_x_str.split_once('=').unwrap().1.parse().unwrap();
            let sensor_y: isize = sensor_y_str.split_once('=').unwrap().1.parse().unwrap();
            sensors.push((sensor_x, sensor_y));

            // parse beacon
            let (beacon_x_str, beacon_y_str) = beacon_str
                .split_once("at ")
                .unwrap()
                .1
                .trim()
                .split_once(", ")
                .unwrap();
            let beacon_x: isize = beacon_x_str.split_once('=').unwrap().1.parse().unwrap();
            let beacon_y: isize = beacon_y_str.split_once('=').unwrap().1.parse().unwrap();
            beacons.push((beacon_x, beacon_y));
        });

    (sensors, beacons)
}

fn manhattan_distance(p1: &Point, p2: &Point) -> isize {
    (isize::abs_diff(p1.0, p2.0) + isize::abs_diff(p1.1, p2.1)) as isize
}

fn calculate_distances(sensors: &[Point], beacons: &[Point]) -> Vec<isize> {
    let mut distances = Vec::new();

    for i in 0..sensors.len() {
        distances.push(manhattan_distance(&sensors[i], &beacons[i]));
    }

    distances
}

fn process(sensors: &[Point], beacons: &[Point], target_row: isize) -> isize {
    let distances = calculate_distances(sensors, beacons);

    // positions on the target line where there is a beacon
    let mut beacons_xs_in_target_row = Vec::new();
    for beacon in beacons {
        if beacon.1 == target_row {
            beacons_xs_in_target_row.push(beacon.0);
        }
    }

    // get the ranges
    let mut ranges = Vec::new();
    for i in 0..beacons.len() {
        let dx = distances[i] - (target_row - sensors[i].1).abs();

        if dx > 0 {
            ranges.push((sensors[i].0 - dx, sensors[i].0 + dx));
        }
    }

    let mut out = 0;
    let min_x = ranges.iter().map(|el| el.0).min().unwrap();
    let max_x = ranges.iter().map(|el| el.1).max().unwrap();

    // search the x that are in the ranges
    for x in min_x..=max_x {
        if !beacons_xs_in_target_row.contains(&x) {
            for (left, right) in &ranges {
                if left <= &x && &x <= right {
                    out += 1;
                    break;
                }
            }
        }
    }

    out
}

fn part_one(path: &str, target_row: isize) -> isize {
    let (sensors, beacons) = parse_input(path);

    process(&sensors, &beacons, target_row)
}

fn part_two(path: &str) -> usize {
    let (sensors, beacons) = parse_input(&path);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT, 10), 26);
    }

    #[ignore]
    fn _test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 93);
    }
}
