use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input = read_to_string(format!("./src/input.txt")).unwrap();

    let finds: Vec<&str> = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap().find_iter(&input).map(|x| x.as_str()).collect();

    let mut total = 0;

    for find in finds {
        let ints = Regex::new(r"\d{1,3}").unwrap().find_iter(find).map(|x| x.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        total += ints[0] * ints[1];
    }

    println!("{}", total);
}
