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

    let mut total = 0;

    for num in raw1 {
        let mut a = 0;
        raw2.iter().for_each(|n| {
            if num == n.clone() {
                a += 1;
            }
        });
        total += num * a;
    }

    println!("{}", total);
}