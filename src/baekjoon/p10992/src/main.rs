fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    
    for i in 0..n {
        for _ in 0..(n - i - 1) {
            print!(" ");
        }
        print!("*");
        if i == 0 {
            println!();
            continue;
        }

        if i == n - 1 {
            for _ in 0..(2 * i - 1) {
                print!("*");
            }
        } else {
            for _ in 0..(2 * i - 1) {
                print!(" ");
            }
        }
        
        println!("*");
    }
}
