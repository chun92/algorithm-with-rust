fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn fill_grid(m: i32, n: i32, k: i32) -> Vec<Vec<(i32, i32)>> {
    let mut grid = vec![vec![(0, 0); n as usize]; m as usize];
    for _ in 0..k {
        let args = read_line_as_numbers();
        let (x, y) = (args[0], args[1]);
        grid[x as usize][y as usize] = (1, 0);
    }
    grid
}

fn dfs(grid: &mut Vec<Vec<(i32, i32)>>, x: i32, y: i32, m: i32, n: i32) {
    if x < 0 || x >= m || y < 0 || y >= n {
        return;
    }
    if grid[x as usize][y as usize].0 == 0 {
        return;
    }
    if grid[x as usize][y as usize].1 == 1 {
        return;
    }
    grid[x as usize][y as usize].1 = 1;
    
    dfs(grid, x - 1, y, m, n);
    dfs(grid, x + 1, y, m, n);
    dfs(grid, x, y - 1, m, n);
    dfs(grid, x, y + 1, m, n);
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let args = read_line_as_numbers();
        let (m, n, k) = (args[0], args[1], args[2]);
        let mut grid = fill_grid(m, n, k);
        let mut count = 0;
        for x in 0..m {
            for y in 0..n {
                if grid[x as usize][y as usize].0 == 1 && grid[x as usize][y as usize].1 == 0 {
                    dfs(&mut grid, x, y, m, n);
                    count += 1;
                }
            }
        }
        println!("{}", count);
    }
}
