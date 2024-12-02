use std::fs::read_to_string;

fn main() {
    let input = read_to_string(format!("./src/input.txt")).unwrap();
    
    let mut safe_line_count = 0;

    input.split("\n").for_each(|line| {
        let values = line.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        if is_report_safe(&values) {
            safe_line_count += 1;
        } else {
            // Check if removing a single level would make the report safe
            for i in 0..values.len() {
                let mut new_report = values.clone();
                new_report.remove(i);
                if is_report_safe(&new_report) {
                    safe_line_count += 1;
                    break;
                }
            }
        }
    });

    println!("{}", safe_line_count);
}

fn is_report_safe(values: &Vec<i32>) -> bool {
    let increase: Vec<i32> = vec![1, 2, 3];
    let decrease: Vec<i32> = vec![-1, -2, -3];

    let value_gap = values.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();

    value_gap.iter().all(|x| increase.contains(x)) || value_gap.iter().all(|x| decrease.contains(x))
}