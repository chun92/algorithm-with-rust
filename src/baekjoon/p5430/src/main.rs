use std::collections::VecDeque;

fn read_line_as<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().unwrap()
}

fn parse_array(s: &str) -> VecDeque<i32> {
    let split = s.trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',');

    if split.clone().count() == 1 && split.clone().next().unwrap() == "" {
        return VecDeque::new();
    }

    split.map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

fn do_command(commands: &str, array: &mut VecDeque<i32>) -> Result<VecDeque<i32>, String> {
    let mut is_reverse = false;
    for c in commands.chars() {
        match c {
            'R' => is_reverse = !is_reverse,
            'D' => {
                if array.is_empty() {
                    return Err("error".to_string());
                }
                if is_reverse {
                    array.pop_back();
                } else {
                    array.pop_front();
                }
            }
            _ => (),
        }
    }

    let result = {
        if is_reverse {
            array
                .iter()
                .rev()
                .map(|x| *x)
                .collect::<VecDeque<i32>>()
        } else {
            array
                .iter()
                .map(|x| *x)
                .collect::<VecDeque<i32>>()
        }
    };
    Ok(result)
}

fn print_array(array: &VecDeque<i32>) {
    let mut result = String::new();
    result.push('[');
    for (i, x) in array.iter().enumerate() {
        if i != 0 {
            result.push(',');
        }
        result.push_str(&x.to_string());
    }
        
    result.push(']');
    println!("{}", result);
}

fn main() {
    let n: usize = read_line_as();
    for _ in 0..n {
        let commands = read_line_as::<String>();
        let _size: usize = read_line_as();
        let array_str = read_line_as::<String>();
        let mut array = parse_array(&array_str);
        let result = do_command(&commands, &mut array);
        match result {
            Ok(array) => {
                print_array(&array);
            },
            Err(_) => {
                println!("error");
            }
        }
    }
}
