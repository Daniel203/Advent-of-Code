use std::fs;

fn main() {
    let res1 = res1();
    let res2 = res2();

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn res1() -> i32 {
    let contents = fs::read_to_string("src/data1.txt").unwrap();

    let mut sum: i32 = 0;

    for line in contents.lines() {
        let val1: i32 = Into::<i32>::into(line.as_bytes()[0]) - 64;
        let val2: i32 = Into::<i32>::into(line.as_bytes()[2]) - 87;

        let res: i32 = (val2 - val1 + 4) % 3;

        sum += val2 + (res * 3);
    }

    return sum;
}

fn res2() -> i32 {
    let contents = fs::read_to_string("src/data1.txt").unwrap();

    let mut sum: i32 = 0;

    for line in contents.lines() {
        let val1: i32 = Into::<i32>::into(line.as_bytes()[0]) - 65;
        let val2: i32 = (Into::<i32>::into(line.as_bytes()[2]) - 88) * 3;

        let val3: i32 = match val2 {
            0 => val1 - 1,
            3 => val1,
            6 => val1 + 1,
            _ => 0,
        };

        sum += val2 + (val3 + 3) % 3 + 1;
    }

    return sum;
}
