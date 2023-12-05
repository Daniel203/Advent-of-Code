use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = (Vec<usize>, Vec<Vec<(usize, usize, usize)>>);

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let mut parsed_input: InputType = (vec![], vec![]);

    input
        .split("\n\n")
        .map(|x| x.to_string())
        .enumerate()
        .for_each(|(line_n, data)| {
            if line_n == 0 {
                // seeds
                let nums = data
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                parsed_input.0 = nums;
            } else {
                // maps
                let numbers = data
                    .split("\n")
                    .skip(1)
                    .filter(|x| x.len() > 0)
                    .map(|x| {
                        let nums = x
                            .split(" ")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        return (nums[0], nums[1], nums[2]);
                    })
                    .collect::<Vec<(usize, usize, usize)>>();

                parsed_input.1.push(numbers);
            }
        });

    return parsed_input;
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);
    let (seeds, maps) = data;

    let output = seeds
        .iter()
        .map(|&seed| {
            // iterate over the list of maps until the last one
            maps.iter().fold(seed, |seed, map| {
                // every map can contain multiple ranges, so we need to iterate over them
                // and find the one that contains the seed
                map.iter()
                    .find(|&&(_, start, len)| (start..start + len).contains(&seed))
                    .map(|(dest, start, _)| dest + (seed - start))
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap();

    return output;
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);
    let maps = data.1;

    let mut seeds = data
        .0
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1]))
        .collect::<Vec<(usize, usize)>>();

    for map in maps {
        let mut new = Vec::new();
        while !seeds.is_empty() {
            let (seed_start, seed_end) = seeds.pop().unwrap();

            // if the seed is not contained in any of the ranges, we can just add it to the new list
            let contained = map
                .iter()
                .all(|&(_, start, len)| seed_start >= start + len || seed_end <= start);

            if contained {
                new.push((seed_start, seed_end));
                continue;
            }

            for (dest, start, len) in &map {
                let overlap_start = seed_start.max(*start);
                let overlap_end = seed_end.min(start + len);
                // if the overlap is not empty, we need to add it to the new list
                if overlap_start < overlap_end {
                    // add the overlap using the destination values
                    new.push((overlap_start - start + dest, overlap_end - start + dest));

                    // add the remaining parts of the seed to the seeds list so we can check them
                    if overlap_start > seed_start {
                        seeds.push((seed_start, overlap_start));
                    }
                    if seed_end > overlap_end {
                        seeds.push((overlap_end, seed_end));
                    }
                }
            }
        }
        seeds = new;
    }

    let min_seed = seeds.iter().map(|(a, _)| a).min().unwrap();
    return *min_seed;
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 46);
    }
}
