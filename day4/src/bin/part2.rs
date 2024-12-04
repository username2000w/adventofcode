use std::fs::read_to_string;

fn main() {
    let input = read_to_string(format!("./src/input.txt")).unwrap();

    let mut grid = vec![];
    let mut i = 0;
    input.split("\r\n").for_each(|line| {
        grid.push(vec![]);
        for c in line.chars() {
            grid[i].push(c);
        }
        i += 1;
    });

    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A' {
                if i >= 1 && j >= 1 && i < grid.len() - 1 && j < grid[i].len() - 1 {
                    if ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
                        || (grid[i + 1][j + 1] == 'M' && grid[i - 1][j - 1] == 'S'))
                        && ((grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
                            || (grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S'))
                    {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("{}", total);
}
