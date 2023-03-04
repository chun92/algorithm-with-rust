use std::io;
use io::Write;

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn copy_stars(into_vec: &mut Vec<Vec<bool>>, from_vec: &Vec<Vec<bool>>, x: usize, y: usize) {
    for i in 0..from_vec.len() {
        into_vec[x + i][y..y + from_vec.len()].copy_from_slice(&from_vec[i][..]);
    }
}

fn print_stars(n: usize) -> Vec<Vec<bool>> {
    let mut temp: Vec<Vec<bool>> = vec![vec![false; n]; n];
    if n == 1 {
        temp[0][0] = true;
    } else {
        for i in 0..3 {
            for j in 0.. 3{
                if i == 1 && j == 1 {
                    continue;
                }

                let vec = print_stars(n / 3);
                copy_stars(&mut temp, &vec, i * n / 3, j * n / 3);
            }
        }
    }
    temp
}
fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = read_line_as_number();
    let stars = print_stars(n);
    for i in 0..n {
        for j in 0..n {
            if stars[i][j] {
                write!(out, "{}", '*').unwrap();
            } else {
                write!(out, "{}", ' ').unwrap();
            }
        }
        writeln!(out).unwrap();
    }
}
