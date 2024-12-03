use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input = read_to_string(format!("./src/input.txt")).unwrap();

    let finds: Vec<&str> = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap().find_iter(&input).map(|x| x.as_str()).collect();

    let mut total = 0;
    let mut enable = true;

    for find in finds {
        if find == "do()" {
            enable = true;
        }

        if find == "don't()" {
            enable = false;
        }

        if enable {
            let ints = Regex::new(r"\d{1,3}").unwrap().find_iter(find).map(|x| x.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            
            if ints.len() == 2 {
                total += ints[0] * ints[1];
            }
        }
    }

    println!("{}", total);
}
