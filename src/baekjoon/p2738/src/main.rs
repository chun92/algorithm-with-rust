use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn build_matrix(n: usize) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let vec = read_line_as_numbers();
        matrix.push(vec);
    }
    matrix
}

fn main() {
    let arguements = read_line_as_numbers();
    let (n, m) = (arguements[0] as usize, arguements[1] as usize);

    let matrix_a = build_matrix(n);
    let matrix_b = build_matrix(n);

    for i in 0..n {
        for j in 0..m {
            if j != 0 {
                print!(" ");
            }
            print!("{}", matrix_a[i as usize][j as usize] + matrix_b[i as usize][j as usize]);
        }
        println!();
    }
}
