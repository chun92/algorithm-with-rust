use std::collections::HashSet;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut set = HashSet::new();

    let mut i = 1;
    loop {
        let num = i * i;
        if num == n {
            println!("{}", 1);
            return;
        } else if num < n {
            set.insert(num);
        } else {
            break;
        }
        i += 1;
    }

    let mut second_set = HashSet::new();
    let mut result = false;
    set
        .iter()
        .for_each(|&x| set.iter()
            .for_each(|&y| {
                let num = x + y;
                if result {
                    return;
                }
                if num == n {
                    println!("{}", 2);
                    result = true;
                    return;
                } else if num < n {
                    second_set.insert(num);
                }
            }));

    if result {
        return;
    }
    
    let mut third_set = HashSet::new();
    let mut result = false;
    second_set
        .iter()
        .for_each(|&x| set.iter()
            .for_each(|&y| {
                let num = x + y;
                if result {
                    return;
                }
                if num == n {
                    println!("{}", 3);
                    result = true;
                    return;
                } else if num < n {
                    third_set.insert(num);
                }
            }));
            
    if result {
        return;
    }

    println!("{}", 4);
}
