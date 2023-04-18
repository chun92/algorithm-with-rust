fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();

    let mut result = Vec::new();
    numbers
        .iter()
        .for_each(|x| {
            match result.last() {
                Some(y) => {
                    if x > y {
                        result.push(*x);
                    } else {
                        let index = result.partition_point(|z| z < x);
                        result[index] = *x;
                    }
                },
                None => result.push(*x),
            }
        });
    
    println!("{}", result.len());
}
