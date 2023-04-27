fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();
    let to_remove = read_line_as_string()
        .chars()
        .collect::<Vec<_>>();

    let mut result: Vec<(char, u8)> = Vec::new();
    let mut current_index = 0;
    str
        .chars()
        .for_each(|c| {
            if c == to_remove[current_index] {
                current_index += 1;
                result.push((c, current_index as u8));
            } else if c == to_remove[0] {
                current_index = 1;
                result.push((c, current_index as u8));
            } else {
                current_index = 0;
                result.push((c, current_index as u8));
            }

            if current_index == to_remove.len() {
                for _ in 0..to_remove.len() {
                    result.pop();
                }
                match result.last() {
                    Some((_, index)) => current_index = *index as usize,
                    None => current_index = 0,
                }
            }
        });

    let result = result
        .iter()
        .map(|(c, _)| c)
        .collect::<String>();

    if result.len() == 0 {
        println!("FRULA");
    } else {
        println!("{}", result);
    }
}