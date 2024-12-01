use std::fs::read_to_string;

fn main() {
    let input = read_to_string(format!("./src/input.txt")).unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut raw1: Vec<u32> = vec![];
    let mut raw2: Vec<u32> = vec![];

    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        raw1.push(split[0].parse().unwrap());
        raw2.push(split[1].parse().unwrap());
    }

    raw1.sort();
    raw2.sort();

    let mut total = 0;

    for i in 0..raw1.len() {
        if raw1[i] >= raw2[i] {
            total += raw1[i] - raw2[i];
        } else {
            total += raw2[i] - raw1[i];
        }
    }

    println!("{}", total);
}