use std::fs::read_to_string;

fn main() {
    let input = read_to_string(format!("./src/input.txt")).unwrap();

    let mut safe_line_count = 0;

    let increase: Vec<i32> = vec![1, 2, 3];
    let decrease: Vec<i32> = vec![-1, -2, -3];

    input.split("\n").for_each(|line| {
        let values = line.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        let value_gap = values.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();

        if value_gap.iter().all(|x| increase.contains(x)) || value_gap.iter().all(|x| decrease.contains(x)) {
            safe_line_count += 1;
        }
    });

    println!("{}", safe_line_count);
}
