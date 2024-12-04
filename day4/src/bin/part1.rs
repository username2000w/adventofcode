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
            if grid[i][j] == 'X' {
                if i >= 3 {
                    if grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
                        total += 1;
                    }
                }
                if j >= 3 {
                    if grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
                        total += 1;
                    }
                }
                if i >= 3 && j >= 3 {
                    if grid[i - 1][j - 1] == 'M' && grid[i - 2][j - 2] == 'A' && grid[i - 3][j - 3] == 'S' {
                        total += 1;
                    }
                }
                if i < grid.len() - 3 {
                    if grid[i + 1][j] == 'M' && grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S' {
                        total += 1;
                    }
                }
                if j < grid[i].len() - 3 {
                    if grid[i][j + 1] == 'M' && grid[i][j + 2] == 'A' && grid[i][j + 3] == 'S' {
                        total += 1;
                    }
                }
                if i < grid.len() - 3 && j < grid[i].len() - 3 {
                    if grid[i + 1][j + 1] == 'M' && grid[i + 2][j + 2] == 'A' && grid[i + 3][j + 3] == 'S' {
                        total += 1;
                    }
                }
                if i >= 3 && j < grid[i].len() - 3 {
                    if grid[i - 1][j + 1] == 'M' && grid[i - 2][j + 2] == 'A' && grid[i - 3][j + 3] == 'S' {
                        total += 1;
                    }
                }
                if i < grid.len() - 3 && j >= 3 {
                    if grid[i + 1][j - 1] == 'M' && grid[i + 2][j - 2] == 'A' && grid[i + 3][j - 3] == 'S' {
                        total += 1;
                    }
                }

            }
        }
    }

    println!("{}", total);
}