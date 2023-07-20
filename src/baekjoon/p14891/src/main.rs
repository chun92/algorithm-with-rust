fn read_line_as_num() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .chars()
        .fold(0, |acc, c| acc << 1 | c.to_digit(10).unwrap() as u8)
}

fn read_line_as_numbers() -> Vec<i8> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn rotate(x: &mut u8, dirrection: i8) {
    if dirrection < 0 {
        let temp = (*x << 1) | ((*x & 0b10000000) >> 7);
        *x = temp;
    } else {
        let temp = (*x >> 1) | ((*x & 0b00000001) << 7);
        *x = temp;
    }
}

// 시계 방향 회전이면 >> 1
// 반시계 방향 회전이면 << 1

fn check_left(source: &u8, target: &u8) -> bool {
    let source_pole = (*source & 0b00000010) >> 1;
    let target_pole = (*target & 0b00100000) >> 5;
    if source_pole == target_pole {
        return false;
    } else {
        return true;
    }
}

fn check_right(source: &u8, target: &u8) -> bool {
    let source_pole = (*source & 0b00100000) >> 5;
    let target_pole = (*target & 0b00000010) >> 1;
    if source_pole == target_pole {
        return false;
    } else {
        return true;
    }
}

fn move_all(x: i8, dir: i8, x1: &mut u8, x2: &mut u8, x3: &mut u8, x4: &mut u8) {
    if x == 1 {
        let check_x2 = check_right(x1, x2);
        let check_x3 = check_right(x2, x3);
        let check_x4 = check_right(x3, x4);
        rotate(x1, dir);
        if check_x2 {
            rotate(x2, dir * -1);
        } else {
            return;
        }
        if check_x3 {
            rotate(x3, dir);
        } else {
            return;
        }
        if check_x4 {
            rotate(x4, dir * -1);
        } else {
            return;
        }

    } else if x == 2 {
        let check_x1 = check_left(x2, x1);
        let check_x3 = check_right(x2, x3);
        let check_x4 = check_right(x3, x4);
        rotate(x2, dir);
        if check_x1 {
            rotate(x1, dir * -1);
        }
        if check_x3 {
            rotate(x3, dir * -1);
        } else {
            return;
        }
        if check_x4 {
            rotate(x4, dir);
        } else {
            return;
        }
    } else if x == 3 {
        let check_x1 = check_left(x2, x1);
        let check_x2 = check_left(x3, x2);
        let check_x4 = check_right(x3, x4);
        rotate(x3, dir);
        if check_x4 {
            rotate(x4, dir * -1);
        }

        if check_x2 {
            rotate(x2, dir * -1);
        } else {
            return;
        }
        if check_x1 {
            rotate(x1, dir);
        } else {
            return;
        }
    } else if x == 4 {
        let check_x1 = check_left(x2, x1);
        let check_x2 = check_left(x3, x2);
        let check_x3 = check_left(x4, x3);
        rotate(x4, dir);
        if check_x3 {
            rotate(x3, dir * -1);
        } else {
            return;
        }
        if check_x2 {
            rotate(x2, dir);
        } else {
            return;
        }
        if check_x1 {
            rotate(x1, dir * -1);
        } else {
            return;
        }
    }
}

fn main() {
    let mut x1 = read_line_as_num();
    let mut x2 = read_line_as_num();
    let mut x3 = read_line_as_num();
    let mut x4 = read_line_as_num();

    let n = read_line_as_numbers()[0] as usize;
    for _i in 0..n {
        let (x, dir) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        move_all(x, dir, &mut x1, &mut x2, &mut x3, &mut x4);
    }

    let sum = ((x1 & 0b10000000) >> 7) * 1 
                + ((x2 & 0b10000000) >> 7) * 2
                + ((x3 & 0b10000000) >> 7) * 4
                + ((x4 & 0b10000000) >> 7) * 8;
    println!("{}", sum);
}