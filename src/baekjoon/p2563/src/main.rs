use std::io;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Failed to parse input")
}

fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Failed to parse input"))
        .collect()
}

fn fill_territory(territory: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    for i in x-1..x+9 {
        for j in y-1..y+9 {
            territory[i][j] = true;
        }
    }
}

fn main() {
    let mut vec_of_vec:Vec<Vec<bool>> = Vec::new();
    for _ in 0..100 {
        vec_of_vec.push(vec![false; 100]);
    }

    let n = read_line_as_number();
    for _ in 0..n {
        let (x, y) = {
            let elem = read_line_as_numbers();
            (elem[0] as usize, elem[1] as usize)
        };
        fill_territory(&mut vec_of_vec, x, y);
    }

    let result = vec_of_vec
        .iter()
        .fold(0, |acc, x| {
            acc + x.iter().fold(0, |acc, y| {
                if *y {
                    acc + 1
                } else {
                    acc
                }
            })
        });
    println!("{}", result);
}
