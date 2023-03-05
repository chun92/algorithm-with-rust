fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn get_queen(n: i32, cur: i32, vec: &mut Vec<i32>) -> usize {
    let mut valid = true;
    for i in 0..cur {
        if vec[i as usize] == vec[cur as usize] 
            || (vec[i as usize] as i32 - vec[cur as usize] as i32).abs() == cur - i {
            valid = false;
            break;
        }
    }

    if valid {
        if cur == n - 1 {
            return 1;
        }
        let mut count = 0;
        for i in 0..n {
            vec[(cur + 1) as usize] = i;
            count += get_queen(n, cur + 1, vec);
        }
        count
    } else {
        0
    }
}

fn main() {
    let n = read_line_as_number();
    println!("{}", get_queen(n, -1, &mut vec![0; n as usize]));
}
