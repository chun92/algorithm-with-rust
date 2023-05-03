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
    let mut indexes = Vec::new();
    numbers
        .iter()
        .for_each(|x| {
            match result.last() {
                Some(y) => {
                    if x > y {
                        result.push(*x);
                        indexes.push(result.len() -1);
                    } else {
                        let index = result.partition_point(|z| z < x);
                        result[index] = *x;
                        indexes.push(index);
                    }
                },
                None => {
                    result.push(*x);
                    indexes.push(result.len() -1);
                }
            }
        });

    println!("{}", result.len());
}