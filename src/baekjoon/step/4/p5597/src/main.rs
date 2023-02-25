use std::io;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn main() {
    let mut results = vec![false; 30];

    for _ in 0..28 {
        let index = read_line_as_number() as usize;
        results[index - 1] = true;
    }

    for (i, v) in results.iter().enumerate() {
        if !v {
            println!("{}", i + 1);
        }
    }
}