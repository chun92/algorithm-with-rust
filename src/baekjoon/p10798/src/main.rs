use std::io;

fn read_lines() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .chars()
        .collect()
}

fn main() {
    let vec_of_vec: Vec<Vec<char>> = (0..5)
        .map(|_| read_lines())
        .collect();

    let num = vec_of_vec.len();
    let max_num = vec_of_vec
        .iter()
        .map(|v| v.len())
        .max()
        .unwrap();

    for j in 0..max_num {
        for i in 0..num {
            if let Some(s) = vec_of_vec[i].get(j) {
                print!("{}", s);
            }
        }
    }
    println!()
}